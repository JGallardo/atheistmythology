use chrono::NaiveDateTime;
use chrono::Local;
use serde::Deserialize;
use serde::Serialize;

#[derive(Clone)]
pub struct Post {
    pub id: String,
    pub dt: NaiveDateTime,
    pub image_url: String,
    pub title: String,
    pub text: String,
}

impl Post {
    pub fn new_empty() -> Post {
        Post {
            id: "".to_string(),
            dt: Local::now().naive_local(),
            image_url: "".to_string(),
            title: "".to_string(),
            text: "".to_string(),
        }
    }
}