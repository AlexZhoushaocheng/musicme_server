use crate::{mariadb::Db, minio::Minio};

use rocket::{
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FromRequest, Outcome, Request},
    response::{Flash, Redirect},
};

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

pub struct Music(Vec<u8>);

impl<'l> MusicLib<'l> {
    pub async fn get_music(&self, name: &str) -> Option<Music>{
        println!("name: {}", name);
        let data = self.minio.get_object(format!("/{}",name)).await.ok()?;
        Some(Music(data.bytes().to_vec()))
    }
}