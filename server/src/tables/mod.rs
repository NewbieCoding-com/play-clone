#[cfg(feature =  "dev")]
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
#[cfg(feature =  "prod")]
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};
#[cfg(feature =  "prod")]
use sqlx::{migrate::MigrateDatabase, MySql, Pool};
#[cfg(feature =  "dev")]
use sqlx::sqlite::SqliteQueryResult;
use tracing::info;
use crate::config::Config;
use crate::file_path;

pub mod user;
pub mod article;
pub mod todo_item;
pub mod api_entry;


#[cfg(feature =  "dev")]
pub type DBPool = Pool<Sqlite>;
#[cfg(feature =  "dev")]
pub type DBQueryResult = SqliteQueryResult;

#[cfg(feature =  "prod")]
pub type DBPool = Pool<MySql>;
#[cfg(feature =  "prod")]
pub type DBQueryResult = MySqlQueryResult;

#[cfg(feature =  "dev")]
#[macro_export]
macro_rules! get_last_insert_id {
    ($t: expr) => {
        $t.last_insert_rowid()
    }
}
#[cfg(feature =  "prod")]
#[macro_export]
macro_rules! get_last_insert_id {
    ($t: expr) => {
        {
            $t.last_insert_id() as i64
        }
    }
}



#[cfg(feature =   "dev")]
pub async fn init_pool(config : &Config) -> DBPool {

    let db_url: &str = config.database.url.as_str();

    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    let db = SqlitePool::connect(db_url).await.unwrap();
    let result = sqlx::query(include_str!(file_path!("/../doc/db_sqlite.sql"))).execute(&db).await.unwrap();
    // info!("Create  table result: {:?}", result);
    db
}


#[cfg(feature =   "dev")]
pub async fn init_test_pool() -> DBPool {
    let db_test_url = ":memory:";
    let db = SqlitePool::connect(db_test_url).await.unwrap();
    let result = sqlx::query(include_str!(file_path!("/../doc/db_sqlite.sql"))).execute(&db).await.unwrap();
    // info!("Create  table result: {:?}", result);
    db
}



#[cfg(feature =   "prod")]
pub async fn init_pool(config : &Config) -> DBPool {

    let db_url: &str = config.database.url.as_str();

    if !MySql::database_exists(db_url).await.unwrap_or(false) {
        info!("Creating database {}", db_url);
        match MySql::create_database(db_url).await {
            Ok(_) => info!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(db_url).await.unwrap();

    for s in include_str!(file_path!("/../doc/db_mysql.sql")).split(";"){
        if s.trim().is_empty(){
            continue
        }
        let result = sqlx::query(s).execute(&db).await.unwrap();
        // info!("Create  table result: {:?}", result);
    };



    db
}

#[cfg(feature =   "prod")]
pub async fn init_test_pool() -> DBPool {
    const DB_URL: &str = "mysql://localhost:3306/test";

    if MySql::database_exists(DB_URL).await.unwrap_or(false) {
        //delete database
        MySql::drop_database(DB_URL).await.unwrap()
    }
    info!("Creating database {}", DB_URL);
    match MySql::create_database(DB_URL).await {
        Ok(_) => info!("Create db success"),
        Err(error) => panic!("error: {}", error),
    }


    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await.unwrap();

    for s in include_str!(file_path!("/../doc/db_mysql.sql")).split(";"){
        if s.trim().is_empty(){
            continue
        }
        let result = sqlx::query(s).execute(&db).await.unwrap();
        info!("Create  table result: {:?}", result);
    };



    db
}



pub mod users;
