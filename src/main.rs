#[macro_use]
extern crate rocket;

mod login;
mod mariadb;
mod minio;
mod music;
mod musiclib;
mod config;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(minio::stage())
        .attach(mariadb::stage())
        .mount("/session", login::routes())
        .mount("/music", music::routes())
}
