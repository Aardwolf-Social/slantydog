// aardwolf-api/database-api/src/database_main.rs
//
use crate::database_engines::mysql::MysqlConnection;
use crate::database_engines::postgres::PgConnection;
use crate::database_engines::sqlite::SqliteConnection;

pub enum DatabaseConnection {
    PgConnection(PgConnection),
    MysqlConnection(MysqlConnection),
    SqliteConnection(SqliteConnection),
}

pub trait MyConnection {
    fn execute_query(
        &self,
        query: Box<dyn diesel::query_builder::QueryFragment<Self> + 'static>,
    ) -> Result<(), diesel::result::Error>;
    fn execute_transaction(
        &self,
        transaction: Box<dyn diesel::query_builder::QueryFragment<Self> + 'static>,
    ) -> Result<(), diesel::result::Error>;
}
