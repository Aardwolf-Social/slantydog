// mysql

use crate::database_api::{DbHandler, Post, Comment, Error};
use crate::mysql;

pub struct MySqlHandler {
    connection: mysql::PooledConn,
    pool: mysql::Pool,
    connection_url: String,
    is_active: bool,
    schema: Vec<String>,
}

impl DbHandler for MySqlHandler {
    fn get_posts(&self) -> Vec<dyn Post> {
        // implement mysql-specific logic to retrieve posts
        todo!()
    }

    fn create_post(&self, post: Post) -> Result<(), dyn Error> {
        // implement mysql-specific logic to create a post
        todo!()
    }

    fn update_post(&self, post_id: i32, post: Post) -> Result<(), Error> {
        // implement mysql-specific logic to update a post
        todo!()
    }

    fn delete_post(&self, post_id: i32) -> Result<(), Error> {
        // implement mysql-specific logic to delete a post
        todo!()
    }

    fn like_post(&self, post_id: i32) -> Result<(), Error> {
        // implement mysql-specific logic to like a post
        todo!()
    }

    fn comment_post(&self, post_id: i32, comment: Comment) -> Result<(), Error> {
        // implement mysql-specific logic to comment on a post
        todo!()
    }
}
pub struct MySqlPost {
    id: i32,
    content: String,
}

impl Post for MySqlPost {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }
}