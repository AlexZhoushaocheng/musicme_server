use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FromRequest, Outcome, Request},
    response::{Flash, Redirect}, serde::json::Json,
};
use super::mariadb::Db;
use crate::login::User;
use crate::minio;
use crate::musiclib::MusicLib;

#[get("/query/<name>")]
async fn get_music(name:String, user:User, musiclib:MusicLib<'_>)->String{
    let music = musiclib.get_music(name.as_str()).await;

    match music {
        Some(_) =>{format!("music name:{}, user_id:{}",name, user.0)},
        None => {"error".to_string()}
    }
    
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_music]
}