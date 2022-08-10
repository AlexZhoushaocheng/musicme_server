#[macro_use]
extern crate rocket;

// use rocket::{Rocket, Build, State};
use figment::{
    providers::{Format, Json},
    Figment,
};

mod login;
mod mariadb;
mod minio;
mod music;
mod musiclib;
// mod db;
mod config;
use config::MyConf;
use login::User;
use rocket::fairing::AdHoc;

#[get("/")]
async fn test_user(user: User) -> &'static str {
    "lalala"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_ignite("mini", |r| async {
            let conf: MyConf = Figment::new()
                .join(Json::file("config.json"))
                .extract()
                .expect("加载配置错误");

            let minio = minio::Minio::new(
                conf.minio.bucket_name.clone(),
                conf.minio.endpoint.clone(),
                conf.minio.user.clone(),
                conf.minio.password.clone(),
            )
            .await
            .unwrap();

            r.manage(minio).manage(conf)
        }))
        .attach(mariadb::stage())
        .mount("/test_user", routes![test_user])
        .mount("/session", login::routes())
        .mount("/music", music::routes())
}
