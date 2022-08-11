use crate::{
    mariadb::{self, Db},
    minio::Minio,
};

use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};

// 歌曲信息
#[derive(Debug,Deserialize, Serialize,  sqlx::FromRow)]
#[serde(crate = "rocket::serde")]
pub struct Song {
    pub id: u64,
    pub name: String,
    pub ar: String,
    pub al: String,
    pub lyric: String,
}

pub enum SearchType {
    ByName, // 歌曲名称
    ByAr,   // 演唱
    ByAl,   // 专辑
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
        sear_type: SearchType,
    ) -> mariadb::Result<Vec<Song>> {
        let mut conn = self.maria.acquire().await?;
        let sql = format!("select id, name,ar,al, lyric  from music where name like '%{}%'", key);

        match sear_type {
            SearchType::ByName => {
                let songs: Vec<Song> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
                Ok(songs)
            }
            SearchType::ByAl => {
                let songs: Vec<Song> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
                Ok(songs)
            }
            SearchType::ByAr => {
                let songs: Vec<Song> = sqlx::query_as(&sql).fetch_all(&mut conn).await?;
                Ok(songs)
            }
        }

    }
}
