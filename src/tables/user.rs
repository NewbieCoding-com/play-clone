use sqlx::{Error, FromRow};

use crate::tables::{DBPool, DBQueryResult};

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}
pub struct AddUser {
    pub name: String,
}

impl User {
    pub async fn add_user(user: AddUser, pool: &DBPool) -> Result<DBQueryResult, Error> {
        sqlx::query("INSERT INTO users (name) VALUES (?)")
            .bind(&user.name)
            .execute(pool)
            .await
    }

    pub async fn query_users(pool: &DBPool) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT id, name FROM users")
            .fetch_all(pool)
            .await
    }

    pub async fn query_users_by_name(name: &str, pool: &DBPool) -> Result<Vec<User>, Error> {
        sqlx::query_as::<_, User>("SELECT id, name FROM users where name = ?")
            .bind(name)
            .fetch_all(pool)
            .await
    }
}