use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Track{
    pub id: i32,
    pub title: String,
    pub artist: String,
    pub album: String,
    pub duration: i32,
}