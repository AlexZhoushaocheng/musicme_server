// use md5::{Digest};
use rocket::fairing::{self, AdHoc};
// use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::{Build, Rocket};
use rocket_db_pools::{sqlx, Database};
use sqlx::pool::PoolConnection;

// pub type Result<T, E = rocket::response::Debug<sqlx::Error>> = std::result::Result<T, E>;
pub type Result<T, E = String> = std::result::Result<T, E>;

pub fn sqlx_err(e: sqlx::Error) -> String {
    e.to_string()
}

// 用户账户
#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Database)]
#[database("octopus")]
pub struct Db(sqlx::MySqlPool);

impl Db {
    pub async fn get_conn(&self)->Result<PoolConnection<sqlx::MySql>>{
        self
            .acquire()
            .await
            .map_err(sqlx_err)
    }
}

// 初始化书数据库
async fn init_db(rocket: Rocket<Build>) -> fairing::Result {
    println!("init db");
    match Db::fetch(&rocket) {
        Some(db) => {
            let conn = db.acquire().await;
            match conn {
                Ok(mut conn_) => {
                    let _row = sqlx::query("select version()").fetch_one(&mut conn_).await;
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
        // .mount("/sql", routes![list])
    })
}
