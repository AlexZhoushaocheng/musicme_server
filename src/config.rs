
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MariaConf{
    pub ip:String,
    pub port:i16,
    pub user:String,
    pub db_name:String,

}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MinioConf{
    pub endpoint:String,
    pub user:String,
    pub password:String,
    pub bucket_name:String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct MyConf{
    pub maria:MariaConf,
    pub minio:MinioConf
}
