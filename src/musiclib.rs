use crate::{
    mariadb::{self, sqlx_err, Db, User},
    minio::Minio,
};

use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};

// 分页每页的数量
const PAGE_LIMIT: i32 = 2;

// 歌曲信息
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Song {
    pub id: u64,
    pub uuid: String,
    pub name: String,
    pub ar: String,
    pub al: String,
    pub lyric: String,
}

#[derive(Debug)]
pub enum SearchType {
    ByName, // 歌曲名称
    ByAr,   // 演唱
    ByAl,   // 专辑
    Unknown,
}

impl From<&str> for SearchType {
    fn from(t: &str) -> Self {
        let mut s_type = SearchType::Unknown;
        if t == "ByName" {
            s_type = SearchType::ByName
        } else if t == "ByAr" {
            s_type = SearchType::ByAr
        } else if t == "ByAl" {
            s_type = SearchType::ByAl
        }
        s_type
    }
}

pub struct MusicLib<'l> {
    maria: &'l Db,
    minio: &'l Minio,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for MusicLib<'r> {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        Outcome::Success(MusicLib {
            maria: req.rocket().state::<Db>().expect("msg"),
            minio: req.rocket().state::<Minio>().expect("msg"),
        })
    }
}

#[derive(sqlx::FromRow)]
struct Count {
    count: i64,
}

pub struct Music(pub Vec<u8>);

impl<'l> MusicLib<'l> {
    // 获取音乐数据
    pub async fn get_music(&self, name: &str) -> Option<Music> {
        let data = self.minio.get_object(format!("/{}", name)).await.ok()?;
        Some(Music(data.bytes().to_vec()))
    }

    // 搜索歌曲
    pub async fn search_song(
        &self,
        key: &str,
        search_type: SearchType,
    ) -> mariadb::Result<Vec<Song>> {
        let mut conn = self.maria.get_conn().await?;
        let sql = format!(
            "select id,uuid, name,ar,al, lyric  from musicme where name like '%{}%'",
            key
        );

        match search_type {
            SearchType::Unknown => Err(String::from("unknown search type")),
            SearchType::ByName => {
                let songs: Vec<Song> = sqlx::query_as(&sql)
                    .fetch_all(&mut conn)
                    .await
                    .map_err(sqlx_err)?;
                Ok(songs)
            }
            SearchType::ByAl => {
                let songs: Vec<Song> = sqlx::query_as(&sql)
                    .fetch_all(&mut conn)
                    .await
                    .map_err(sqlx_err)?;
                Ok(songs)
            }
            SearchType::ByAr => {
                let songs: Vec<Song> = sqlx::query_as(&sql)
                    .fetch_all(&mut conn)
                    .await
                    .map_err(sqlx_err)?;
                Ok(songs)
            }
        }
    }

    // 根据用户名获取一个用户信息
    pub async fn get_user(&self, username: &str) -> mariadb::Result<User> {
        let mut conn = self.maria.get_conn().await?;
        let user: User = sqlx::query_as("select * from users where username= ?")
            .bind(username)
            .fetch_one(&mut conn)
            .await
            .map_err(sqlx_err)?;
        Ok(user)
    }

    // 注册一个用户
    pub async fn reg_user(&self, username: &str, password: &str) -> mariadb::Result<()> {
        let mut conn = self.maria.get_conn().await?;
        let digest_password = md5::compute(password);
        sqlx::query("insert into user(username, password) values(?,?)")
            .bind(username)
            .bind(format!("{:?}", digest_password))
            .execute(&mut conn)
            .await
            .map_err(sqlx_err)?;
        Ok(())
    }

    // uuid没有被校验是否真实存在
    pub async fn add2favorite(&self, user_id: i64, uuid: &str) -> mariadb::Result<()> {
        // 是否重复添加
        let mut conn = self.maria.get_conn().await?;
        let count: Count = sqlx::query_as(
            "SELECT COUNT(user_id) as count FROM favorite WHERE user_id=? and music_uuid=?",
        )
        .bind(user_id)
        .bind(uuid)
        .fetch_one(&mut conn)
        .await
        .map_err(sqlx_err)?;

        if count.count > 0 {
            return Err(String::from("Already exists"));
        }

        // 插入
        sqlx::query("INSERT INTO `musicme`.`favorite`(`user_id`, `music_uuid`) VALUES (?, ?)")
            .bind(user_id)
            .bind(uuid)
            .execute(&mut conn)
            .await
            .map_err(sqlx_err)?;

        Ok(())
    }

    // 获取favorite表中的数据
    pub async fn my_favorite(&self, user_id: i64) -> mariadb::Result<Vec<Song>> {
        let mut conn = self.maria.get_conn().await?;

        let songs: Vec<Song> = sqlx::query_as("SELECT musicme.* FROM favorite  RIGHT JOIN musicme ON favorite.music_uuid = musicme.uuid WHERE user_id=?").bind(user_id).fetch_all(&mut conn).await.map_err(sqlx_err)?;

        Ok(songs)
    }

    pub async fn remove_favorite(&self, user_id: i64, uuid: &str) -> mariadb::Result<()> {
        let mut conn = self.maria.get_conn().await?;

        sqlx::query(
            "DELETE FROM `musicme`.`favorite` WHERE `user_id` = ? AND `music_uuid` = ? LIMIT 1",
        )
        .bind(user_id)
        .bind(uuid)
        .execute(&mut conn)
        .await
        .map_err(sqlx_err)?;
        Ok(())
    }

    // 分页查询整个音乐库
    pub async fn query_by_page(&self, page_num: i32) -> mariadb::Result<Vec<Song>> {
        let mut conn = self.maria.get_conn().await?;
        let songs: Vec<Song> = sqlx::query_as("SELECT * FROM musicme LIMIT ?,?")
            .bind((page_num - 1) * PAGE_LIMIT)
            .bind(PAGE_LIMIT)
            .fetch_all(&mut conn)
            .await
            .map_err(sqlx_err)?;

        Ok(songs)
    }
}
