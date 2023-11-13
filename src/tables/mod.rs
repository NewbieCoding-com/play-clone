#[cfg(ENV = "dev")]
use sqlx::{migrate::MigrateDatabase, Pool, Sqlite, SqlitePool};
#[cfg(ENV = "prod")]
use sqlx::mysql::{MySqlPoolOptions, MySqlQueryResult};
#[cfg(ENV = "prod")]
use sqlx::{migrate::MigrateDatabase, MySql, Pool};
#[cfg(ENV = "dev")]
use sqlx::sqlite::SqliteQueryResult;
use tracing::info;

pub mod user;
mod article;



#[cfg(ENV = "dev")]
pub type DBPool = Pool<Sqlite>;
#[cfg(ENV = "dev")]
pub type DBQueryResult = SqliteQueryResult;

#[cfg(ENV = "prod")]
pub type DBPool = Pool<MySql>;
#[cfg(ENV = "prod")]
pub type DBQueryResult = MySqlQueryResult;

#[cfg(ENV = "dev")]
pub async fn init_pool() -> DBPool {

    const DB_URL: &str = "sqlite://sqlite.db";

    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();
    let result = sqlx::query(include_str!("db_sqlite.sql")).execute(&db).await.unwrap();
    info!("Create  table result: {:?}", result);
    db
}


#[cfg(ENV = "dev")]
pub async fn init_test_pool() -> DBPool {
    let db_test_url = ":memory:";
    let db = SqlitePool::connect(db_test_url).await.unwrap();
    let result = sqlx::query(include_str!("db_sqlite.sql")).execute(&db).await.unwrap();
    info!("Create  table result: {:?}", result);
    db
}



#[cfg(ENV = "prod")]
pub async fn init_pool() -> DBPool {

    const DB_URL: &str = "mysql://localhost:3306/play";

    if !MySql::database_exists(DB_URL).await.unwrap_or(false) {
        info!("Creating database {}", DB_URL);
        match MySql::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    }

    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await.unwrap();

    for s in include_str!("db_mysql.sql").split(";"){
        if s.trim().is_empty(){
            continue
        }
        let result = sqlx::query(s).execute(&db).await.unwrap();
        info!("Create  table result: {:?}", result);
    };



    db
}

#[cfg(ENV = "prod")]
pub async fn init_test_pool() -> DBPool {
    const DB_URL: &str = "mysql://localhost:3306/test";

    if MySql::database_exists(DB_URL).await.unwrap_or(false) {
        //delete database
        MySql::drop_database(DB_URL).await.unwrap()
    }
    info!("Creating database {}", DB_URL);
    match MySql::create_database(DB_URL).await {
        Ok(_) => println!("Create db success"),
        Err(error) => panic!("error: {}", error),
    }


    let db = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(DB_URL).await.unwrap();

    for s in include_str!("db_mysql.sql").split(";"){
        if s.trim().is_empty(){
            continue
        }
        let result = sqlx::query(s).execute(&db).await.unwrap();
        info!("Create  table result: {:?}", result);
    };



    db
}


