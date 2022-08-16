use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    request::{self, FromRequest, Outcome, Request},
};

use super::musiclib::MusicLib;
use rocket::serde::{json::Json, Deserialize, Serialize};

// #[derive(Default)]
pub struct User {
    pub id: i64,
    pub username: String,
}

// #[derive(Debug)]
// pub enum ApiKeyError {
//     Missing,
//     Invalid,
// }

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let db_out = req.guard::<MusicLib>().await;

        let db = match db_out {
            Outcome::Success(db) => db,
            Outcome::Failure(e) => return Outcome::Failure(e),
            Outcome::Forward(f) => return Outcome::Forward(f),
        };

        let username = req
            .cookies()
            .get_private("user_id")
            .and_then(|cookie| Some(cookie.value().to_string()));
        let user_id = username.unwrap();
        let info_ret = db.get_user(user_id.as_str()).await.map_err(|err| err);

        match info_ret {
            Ok(info) => Outcome::Success(User {
                id: info.id,
                username: info.username,
            }),
            Err(_) => Outcome::Failure((Status::BadRequest, ())),
        }
        // .map(User).into_outcome((Status::Unauthorized, ()))
    }
}

pub struct Admin(String);

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Admin {
//     type Error = ();
//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         let user = req.guard::<User>().await;
//         let ad = user.map(|u|{Admin(u.0)});
//         ad
//     }
// }

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
async fn post_login(
    jar: &CookieJar<'_>,
    login: Form<Login<'_>>,
    db: MusicLib<'_>,
) -> Json<Res<'static>> {
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

#[get("/logout")]
fn logout(jar: &CookieJar<'_>) {
    jar.remove_private(Cookie::named("user_id"))
}

// #[get("/test")]
// fn test(jar: &CookieJar<'_>) -> &'static str {
//     let user_id = jar.get_private("user_id");
//     match user_id {
//         None => "failed",
//         Some(_) => "ok",
//     }
// }

pub fn routes() -> Vec<rocket::Route> {
    routes![post_login, logout]
}
