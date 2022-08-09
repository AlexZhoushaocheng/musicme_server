use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    outcome::IntoOutcome,
    request::{self, FromRequest, Outcome, Request},
    response::{Flash, Redirect},
};

use super::mariadb::Db;
use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket_db_pools::{sqlx, Connection, Database};

pub struct User<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User<'r> {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        // let s = ;
            
            Outcome::Success(User(req.cookies()
            .get_private("user_id").unwrap().value()))
            
    }
}

pub struct Admin(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies()
            .get_private("user_id")
            .and_then(|id| Some(id.value().to_string()))
            .map(Admin)
            .or_forward(())
    }
}

// 登录Form
#[derive(FromForm)]
struct Login<'r> {
    username: &'r str,
    password: &'r str,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Res<'r> {
    success: bool,
    msg: &'r str,
}

#[post("/login", data = "<login>")]
async fn post_login(jar: &CookieJar<'_>, login: Form<Login<'_>>, db: &Db) -> Json<Res<'static>> {
    let user = db.get_user(login.username).await;
    let d_password = md5::compute(login.password);

    match user {
        Ok(user) => {
            if user.password == format!("{:?}", d_password) {
                jar.add_private(Cookie::new("user_id", user.username));
                Json(Res {
                    success: true,
                    msg: "",
                })
            } else {
                Json(Res {
                    success: false,
                    msg: "Invalid username/password",
                })
            }
        }
        Err(_) => Json(Res {
            success: false,
            msg: "Invalid username/password",
        }),
    }
}

#[get("/test")]
fn test(jar: &CookieJar<'_>) -> &'static str {
    let user_id = jar.get_private("user_id");
    match user_id {
        None => "failed",
        Some(_) => "ok",
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![post_login, test]
}
