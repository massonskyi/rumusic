use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Playlist {
    pub id: i32,
    pub name: String,
    pub owner_id: i32,
    pub track_ids: Vec<i32>,
}