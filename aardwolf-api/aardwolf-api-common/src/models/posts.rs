// posts.rs
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}
pub struct PostData {
    pub title: String,
    pub content: String,
}