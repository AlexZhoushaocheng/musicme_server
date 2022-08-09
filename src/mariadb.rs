use md5::{Digest};
use rocket::fairing::{self, AdHoc};
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{futures, Build, Rocket};
use rocket_db_pools::{sqlx, Connection, Database};

type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;

#[derive(Database)]
#[database("octopus")]
pub struct Db(sqlx::MySqlPool);

impl Db {
    pub async fn get_user(&self, username: &str) -> Result<User> {
        let mut conn = self.acquire().await?;
        let user: User = sqlx::query_as("select * from user where username= ?")
            .bind(username)
            .fetch_one(&mut conn)
            .await?;
        Ok(user)
    }

    pub async fn reg_user(&self, username: &str, password: &str) -> Result<()> {
        let mut conn = self.acquire().await?;
        let digest_password = md5::compute(password);
        sqlx::query("insert into user(username, password) values(?,?)")
            .bind(username)
            .bind(format!("{:?}", digest_password))
            .execute(&mut conn)
            .await?;
        Ok(())
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}



#[derive(Debug, sqlx::FromRow)]
struct Person {
    id: i32,
    name: String,
    sex: i32,
}

#[get("/")]
async fn list(mut db: Connection<Db>) -> Result<String> {
    let p: Vec<Person> = sqlx::query_as("select * from music")
        .fetch_all(&mut *db)
        .await?;
    println!("{:?}", p);
    Ok("list".to_string())
}

// 初始化书数据库
async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    println!("init db");
    match Db::fetch(&rocket) {
        Some(db) => {
            let conn = db.acquire().await;
            match conn {
                Ok(mut conn_) => {
                    let row = sqlx::query("select version()").fetch_one(&mut conn_).await;
                    Ok(rocket)
                }
                Err(_) => Err(rocket),
            }
        }
        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Mariadb", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("Init DB", init_db))
            .mount("/sql", routes![list])
    })
}
