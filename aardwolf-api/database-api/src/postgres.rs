// postgres.rs

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use crate::database_api::DbHandler;

pub struct PostgresHandler {
    connection: PgConnection,
    pool: Pool<ConnectionManager<PgConnection>>,
    connection_url: String,
    is_active: bool,
    schema: Vec<String>,
}

impl DbHandler for PostgresHandler {
    fn get_posts(&self) -> Vec<Post> {
        // implement postgres-specific logic to retrieve posts
        todo!()
    }

    fn create_post(&self, post: Post) -> Result<(), Error> {
        // implement postgres-specific logic to create a post
        todo!()
    }

    fn update_post(&self, post_id: i32, post: Post) -> Result<(), Error> {
        // implement postgres-specific logic to update a post
        todo!()
    }

    fn delete_post(&self, post_id: i32) -> Result<(), Error> {
        // implement postgres-specific logic to delete a post
        todo!()
    }

    fn like_post(&self, post_id: i32) -> Result<(), Error> {
        // implement postgres-specific logic to like a post
        todo!()
    }

    fn comment_post(&self, post_id: i32, comment: Comment) -> Result<(), Error> {
        // implement postgres-specific logic to comment on a post
        todo!()
    }
}

impl Error for PgSqlError {
    fn get_message(&self) -> String {
        self.message.clone()
    }
}

pub struct PgSqlError {
    message: String,
}

pub struct PgSqlPost {
    id: i32,
    content: String,
}

impl Post for PgSqlPost {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_content(&self) -> String {
        self.content.clone()
    }
}

pub struct PgSqlComment {
    id: i32,
    content: String,
}      

impl Comment for PgSqlComment {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_content(&self) -> String { 
        self.content.clone()
    }
}