#[macro_use] extern crate rocket;

use rocket::{Rocket, Build, State};
use figment::{Figment, providers::{Format, Toml, Json, Env}};
use rocket::serde::{Serialize, Deserialize};

mod db;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MariaConf{
    ip:String,
    port:i16,
    user:String,
    db_name:String,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MinioConf{
    ip:String,
    port:i16,
    user:String,
    basket_name:String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct MyConf{
    maria:MariaConf,
    minio:MinioConf
}

#[launch]
fn rocket() -> _ {
    let conf:MyConf = Figment::new().join(Json::file("config.json")).extract().expect("加载配置错误");
    println!("{:?}", conf.maria.ip);

    rocket::build().manage(conf)
}