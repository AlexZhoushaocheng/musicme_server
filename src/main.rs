#[macro_use]
extern crate rocket;

// use rocket::{Rocket, Build, State};
use figment::{
    providers::{Format, Json},
    Figment,
};

mod minio;
// mod db;
mod config;
use config::MyConf;

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

    rocket::build().manage(conf).manage(minio)
}
