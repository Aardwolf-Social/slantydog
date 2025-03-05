// posts.rs
use serde::{Serialize, Deserialize};
pub trait Post {
    fn get_id(&self) -> i32;
    fn get_title(&self) -> String;
    fn get_content(&self) -> String;
    fn get_created_at(&self) -> String;
    fn get_updated_at(&self) -> String;
}

#[derive(Serialize, Deserialize)]
pub struct PostImpl {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub updated_at: String,
}

impl Post for PostImpl {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_title(&self) -> String {
        self.title.clone()
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }

    fn get_created_at(&self) -> String {
        self.created_at.clone()
    }

    fn get_updated_at(&self) -> String {
        self.updated_at.clone()
    }
}