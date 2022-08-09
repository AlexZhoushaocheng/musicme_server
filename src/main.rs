#[macro_use]
extern crate rocket;

// use rocket::{Rocket, Build, State};
use figment::{
    providers::{Format, Json},
    Figment,
};

mod mariadb;
mod login;
mod minio;
// mod db;
mod config;
use config::MyConf;
use rocket::fairing::AdHoc;
use login::User;

#[get("/")]
async fn test_user(user: User<'_>)-> &'static str{
 
    "lalala"
}

#[launch]
fn rocket() -> _ {
    info!("hello");
    // rocket::tokio::spawn(async move{
    //     minio::minio_test().await;

    // });
    let conf: MyConf = Figment::new()
        .join(Json::file("config.json"))
        .extract()
        .expect("加载配置错误");

    let minio = minio::Minio::new(
        "".into(),
        conf.minio.endpoint.clone(),
        conf.minio.user.clone(),
        conf.minio.password.clone(),
    )
    .unwrap();

    rocket::build().attach(mariadb::stage()).manage(conf).manage(minio).mount("/test_user", routes![test_user]).mount("/session", login::routes())
}
