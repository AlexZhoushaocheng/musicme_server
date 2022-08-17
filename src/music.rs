use crate::login::UserInfo;
use crate::musiclib::{MusicLib, SearchType, Song};
use rocket::serde::{Deserialize, Serialize};
use rocket::{
    http::ContentType,
    serde::json::{self, Json},
};

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResJ {
    success: bool,
    msg: Option<String>,

    data: json::Value,
}

#[get("/query/<uuid>")]
async fn get_music(
    uuid: String,
    _user_info: UserInfo,
    musiclib: MusicLib<'_>,
) -> Option<(ContentType, Vec<u8>)> {
    let v: Vec<&str> = uuid.split('.').collect();
    let music = musiclib.get_music(v[0]).await;

    match music {
        Some(data) => Some((ContentType::MP4, data.0)),
        None => None,
    }
}

#[get("/search/<key>/<by>")]
async fn search_song(
    _user_info: UserInfo,
    musiclib: MusicLib<'_>,
    key: &str,
    by: &str,
) -> Json<ResJ> {
    match musiclib.search_song(key, SearchType::from(by)).await {
        Ok(songs) => {
            let v = json::to_value(songs).unwrap();
            Json(ResJ {
                success: true,
                msg: None,
                data: v,
            })
        }
        Err(e) => Json(ResJ {
            success: false,
            msg: Some(e),
            data: json::Value::Null,
        }),
    }
}

#[get("/add2favorite/<uuid>")]
async fn add2favorite(_user_info: UserInfo, uuid: &str, musiclib: MusicLib<'_>) -> Json<ResJ> {
    match musiclib.add2favorite(_user_info.id, uuid).await {
        Ok(_) => {
            // json
            Json(ResJ {
                success: true,
                msg: None,
                data: json::Value::Null,
            })
        }
        Err(e) => Json(ResJ {
            success: false,
            msg: Some(e),
            data: json::Value::Null,
        }),
    }
}

#[get("/getmyfavorite")]
async fn getmyfavorite(_user_info: UserInfo, musiclib: MusicLib<'_>) -> Json<ResJ> {
    match musiclib.my_favorite(_user_info.id).await {
        Ok(songs) => {
            let v = json::to_value(songs).unwrap();
            Json(ResJ {
                success: true,
                msg: None,
                data: v,
            })
        }
        Err(e) => Json(ResJ {
            success: false,
            msg: Some(e),
            data: json::Value::Null,
        }),
    }
}

#[get("/querybypage/<page_num>")]
async fn query_by_page(page_num:i32, _user_info: UserInfo, musiclib: MusicLib<'_>) -> Json<ResJ> {
    match musiclib.query_by_page(page_num).await {
        Ok(songs) => {
            let v = json::to_value(songs).unwrap();
            Json(ResJ {
                success: true,
                msg: None,
                data: v,
            })
        }
        Err(e) => Json(ResJ {
            success: false,
            msg: Some(e),
            data: json::Value::Null,
        }),
    }
}

#[get("/remove_favorite/<uuid>")]
async fn remove_favorite(uuid:&str, _user_info: UserInfo, musiclib: MusicLib<'_>)-> Json<ResJ> {
    match musiclib.remove_favorite(_user_info.id, uuid).await {
        Ok(_) => {
            Json(ResJ {
                success: true,
                msg: None,
                data: json::Value::Null,
            })
        }
        Err(e) => Json(ResJ {
            success: false,
            msg: Some(e),
            data: json::Value::Null,
        }),
    }
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_music, search_song, add2favorite, getmyfavorite, query_by_page, remove_favorite]
}
