use rocket::{
    form::Form,
    http::{Cookie, CookieJar, Status},
    request::{self, FromRequest, Outcome, Request}, serde::json, outcome::IntoOutcome,
};

use super::musiclib::MusicLib;
use rocket::serde::{json::Json, Deserialize, Serialize};



#[derive(Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserInfo {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        req.cookies().get_private("user_info").map(|info_cookie|->UserInfo{
            let user_info: UserInfo = json::from_str(info_cookie.value()).unwrap();
            user_info
        }).into_outcome((Status::Unauthorized, String::from("failed")))
        // match req.cookies().get_private("user_info") {
        //     Some(info_cookie) => {
        //         let user_info: UserInfo = json::from_str(info_cookie.value()).map_err(|e|{
        //             Outcome::Failure((Status::NonAuthoritativeInformation,()))
        //         })?;
        //     return Outcome::Success(user_info)
        // },

        // } 
    }
}

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for UserInfo {
//     type Error = ();
//     async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
//         let db_out = req.guard::<MusicLib>().await;

//         let db = match db_out {
//             Outcome::Success(db) => db,
//             Outcome::Failure(e) => return Outcome::Failure(e),
//             Outcome::Forward(f) => return Outcome::Forward(f),
//         };

//         let username = req
//             .cookies()
//             .get_private("user_id")
//             .and_then(|cookie| Some(cookie.value().to_string()));
//         let user_id = username.unwrap();
//         let info_ret = db.get_user(user_id.as_str()).await.map_err(|err| err);

//         match info_ret {
//             Ok(info) => Outcome::Success(UserInfo {
//                 id: info.id,
//                 username: info.username,
//             }),
//             Err(_) => Outcome::Failure((Status::BadRequest, ())),
//         }
//         // .map(User).into_outcome((Status::Unauthorized, ()))
//     }
// }

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
                let info = UserInfo{id:user.id, username: user.username};
                let str_info = json::to_string(&info).unwrap();
                jar.add_private(Cookie::new("user_info", str_info));
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
    jar.remove_private(Cookie::named("user_info"))
}


// #[catch(401)]
// fn unauthorized(req: &Request)->String {
//     req.f
// }

pub fn routes() -> Vec<rocket::Route> {
    routes![post_login, logout]
}
