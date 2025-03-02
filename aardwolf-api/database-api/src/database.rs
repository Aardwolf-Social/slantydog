//! Database Connectors and API

use diesel::{connection, prelude::*};
use diesel::connection::SimpleConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub enum DbType {
    Mysql,
    Postgres,
    Sqlite,
}

enum DatabaseConnection {
    PgConnection(PgConnection),
    MysqlConnection(MysqlConnection),
    SqliteConnection(SqliteConnection),
}

impl SimpleConnection for DatabaseConnection {
    fn batch_execute(&mut self, query: &str) -> QueryResult<()> {
        match self {
            DatabaseConnection::PgConnection(conn) => conn.batch_execute(query),
            DatabaseConnection::MysqlConnection(conn) => conn.batch_execute(query),
            DatabaseConnection::SqliteConnection(conn) => conn.batch_execute(query),
        }
    }
}



type PgPool = Pool<ConnectionManager<PgConnection>>;
type MysqlPool = Pool<ConnectionManager<MysqlConnection>>;
type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;

struct DatabaseConnectionPool {
    pg_pool: PgPool,
    mysql_pool: MysqlPool,
    sqlite_pool: SqlitePool,
}

impl DatabaseConnectionPool {
    fn new(pg_url: &str, mysql_url: &str, sqlite_url: &str) -> Self {
        let pg_manager = ConnectionManager::<PgConnection>::new(pg_url);
        let mysql_manager = ConnectionManager::<MysqlConnection>::new(mysql_url);
        let sqlite_manager = ConnectionManager::<SqliteConnection>::new(sqlite_url);

        let pg_pool = Pool::new(pg_manager).unwrap();
        let mysql_pool = Pool::new(mysql_manager).unwrap();
        let sqlite_pool = Pool::new(sqlite_manager).unwrap();

        DatabaseConnectionPool {
            pg_pool,
            mysql_pool,
            sqlite_pool,
        }
    }

    fn get_connection(&self, connection_type: DatabaseConnection) -> PooledConnection<ConnectionManager<dyn diesel::Connection>> {
        match connection_type {
            DatabaseConnection::PgConnection(_) => self.pg_pool.get().unwrap(),
            DatabaseConnection::MysqlConnection(_) => self.mysql_pool.get().unwrap(),
            DatabaseConnection::SqliteConnection(_) => self.sqlite_pool.get().unwrap(),
        }
    }
}

pub fn establish_connection(db_type: DbType) -> Result<DatabaseConnection, diesel::result::Error> {
    match db_type {
        DbType::Postgres => Ok(DatabaseConnection::PgConnection(PgConnection::establish("...")?)),
        DbType::Mysql => Ok(DatabaseConnection::MysqlConnection(MysqlConnection::establish("...")?)),
        DbType::Sqlite => Ok(DatabaseConnection::SqliteConnection(SqliteConnection::establish("...")?)),
    }
}
pub trait DbHandler {
    fn get_posts(&self) -> Vec<Box<dyn Post>>;
    fn create_post(&self, post: Box<dyn Post>) -> Result<(), Box<dyn Error>>;
    fn update_post(&self, post_id: i32, post: Box<dyn Post>) -> Result<(), Box<dyn Error>>;
    fn delete_post(&self, post_id: i32) -> Result<(), Box<dyn Error>>;
    fn like_post(&self, post_id: i32) -> Result<(), Box<dyn Error>>;
    fn comment_post(&self, post_id: i32, comment: Box<dyn Comment>) -> Result<(), Box<dyn Error>>;
}

pub trait Post {
    fn get_id(&self) -> i32;
    fn get_content(&self) -> String;
}

pub trait Comment {
    fn get_id(&self) -> i32;
    fn get_content(&self) -> String;
}

pub trait Error: std::error::Error {}

pub fn establish_connection(db_type: DbType) -> Result<Box<dyn Connection<Backend=(), TransactionManager=()>>, diesel::result::Error>{
    match db_type {
        DbType::Mysql => {
            #[cfg(feature = "mysql")]
            {
                MySqlConnection::establish("mysql://localhost/mydb")?
            }
            #[cfg(not(feature = "mysql"))]
            {
                Err(diesel::result::Error::InvalidConnection)
            }
        }
        DbType::Postgres => {
            #[cfg(feature = "postgres")]
            {
                PgConnection::establish("postgres://localhost/mydb")?
            }
            #[cfg(not(feature = "postgres"))]
            {
                Err(diesel::result::Error::InvalidConnection)
            }
        }
        DbType::Sqlite => {
            #[cfg(feature = "sqlite")]
            {
                SqliteConnection::establish("sqlite:///mydb.db")?
            }
            #[cfg(not(feature = "sqlite"))]
            {
                Err(diesel::result::Error::InvalidConnection)
            }
        }
    }
}