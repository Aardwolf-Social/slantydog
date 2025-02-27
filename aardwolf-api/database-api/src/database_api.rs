// database_api.rs (main database API module)
use crate::{mysql, postgres, sqlite};
use crate::mysql::{MySqlHandler, MySqlError};
use crate::postgres::{PostgresHandler, PostgresError};
use crate::sqlite::{SqliteHandler, SqliteError};

pub enum DbType {
    Mysql,
    Postgres,
    Sqlite,
}

pub fn get_db_handler(db_type: DbType) -> Box<dyn DbHandler> {
    match db_type {
        DbType::Mysql => Box::new(mysql::MySqlHandler),
        DbType::Postgres => Box::new(postgres::PostgresHandler),
        DbType::Sqlite => Box::new(sqlite::SqliteHandler),
    }
}

pub trait DbHandler {
    fn get_posts(&self) -> Vec<dyn Post>;
    fn create_post(&self, post: dyn Post) -> Result<(), dyn Error>;
    fn update_post(&self, post_id: i32, post: dyn Post) -> Result<(), dyn Error>;
    fn delete_post(&self, post_id: i32) -> Result<(), dyn Error>;
    fn like_post(&self, post_id: i32) -> Result<(), dyn Error>;
    fn comment_post(&self, post_id: i32, comment: dyn Comment) -> Result<(), dyn Error>;
}

pub trait Post {
    fn get_id(&self) -> i32;
    fn get_content(&self) -> String;
}

pub trait Comment {
    fn get_id(&self) -> i32;
    fn get_content(&self) -> String;
}

pub trait Error {}
