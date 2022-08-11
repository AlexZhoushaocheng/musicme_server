use rocket::{
    http::{ContentType},
};
use crate::login::User;
use crate::musiclib::MusicLib;

#[derive(Responder)]
#[response(status = 200, content_type = "json")]
struct Mp3(Vec<u8>);

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

pub fn routes() -> Vec<rocket::Route> {
    routes![get_music]
}