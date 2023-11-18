use std::ops::{Add, Deref};
use std::sync::Arc;

use axum::{Json, Router};
use axum::extract::{Path, Query, State};
use axum::routing::get;
use tracing::info;

use crate::AppState;
use crate::controller::{R, S};
use shared::models::user::{AddUser, QueryUser, UpdateUser, USER_LIST, UserVo};
use crate::tables::user::User;

pub fn init() -> Router<Arc<AppState>> {
    Router::new()
        .route(USER_LIST, get(user_list))
        .route("/add-user", get(add_user))
        .route("/update-user/:user_id", get(modify_user))
        .route("/delete-user/:user_id", get(delete_user))
}

async fn user_list(Query(q): Query<QueryUser>, state: S) -> R<Json<Vec<User>>> {
    let users = User::query(q, &state.db).await?;
    info!("config : {:?}", state.config);
    Ok(Json(users))
}

async fn add_user(Query(q): Query<AddUser>, state: S) -> R<String> {
    let r = User::insert(q, &state.db).await?;

    Ok(format!("rows affected : {}", r.rows_affected()))
}

async fn modify_user(user_id: Path<i64>, Query(q): Query<UpdateUser>, state: S) -> R<String> {

    let r = User::update(*user_id, q, &state.db).await?;
    let users = User::query(QueryUser { name: "zzp".to_string() }, &state.db).await?;
    Ok(format!("rows affected : {}, now users : {:?}", r.rows_affected(), users))
}

async fn delete_user(Path(user_id): Path<i64>, state: S) -> R<String> {
    // let e = AppError(anyhow!("eerr"));
    // bail!("test error");
    // return Err(anyhow!("test error").into())
    let r = User::delete(user_id, &state.db).await?;
    //
    Ok(format!("rows affected : {}", r.rows_affected()))
}
