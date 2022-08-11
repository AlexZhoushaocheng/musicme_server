use rocket::{
    http::{ContentType}, serde::json::Json,
};
use crate::login::User;
use crate::musiclib::{MusicLib, SearchType, Song};

// #[derive(Responder)]
// #[response(status = 200, content_type = "json")]
// struct Mp3(Vec<u8>);

#[get("/query/<name>")]
async fn get_music(name:String, _user:User, musiclib:MusicLib<'_>)->Option<(ContentType, Vec<u8>)>{
    let music = musiclib.get_music(name.as_str()).await;
    
    match music {
        Some(data) =>{            
            Some((ContentType::MPEG, data.0))
        },
        None => {None}
    }
    
}

#[get("/search/<key>")]
async fn search_song( _user:User, musiclib:MusicLib<'_>, key:&str)->Option<Json<Vec<Song>>>{
     match musiclib.search_song(key, SearchType::ByName).await {
         Ok(songs) => Some(Json(songs)),
         Err(_) => None
     } 
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_music, search_song]
}