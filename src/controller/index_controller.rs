use std::sync::Arc;

use axum::extract::{Query, State};
use axum::response::Html;
use serde::Deserialize;
use serde_json::json;
use tracing::info;

use crate::AppState;
use crate::tables::user::{AddUser, User};

#[derive(Deserialize)]
pub struct Param {
    name: String,
}


pub async fn root(name: Query<Param>, State(state): State<Arc<AppState>>) -> Html<String> {
    // py_tool::test();

    let name = name.0.name;

    let user = AddUser { name: name.to_string() };
    User::add_user(user, &state.db).await.expect("add user error");

    let users = User::query_users_by_name(name.as_str(), &state.db).await.unwrap();

    info!("users : {:?}", users);


    let args = json!({
        "name": name,
        "age": 43,
        "male": true,
    });
    Html::from(state.template_service.render_template("test.html", args))
}


pub async fn htmx_test(name: Query<Param>, State(state): State<Arc<AppState>>) -> Html<String> {
    // py_tool::test();
    let top = state.template_service.render_template("top.html", json!({}));
    let bottom = state.template_service.render_template( "bottom.html", json!({}));

    let args = json!({
        "server": "rust play server",
        "top_html": top,
        "bottom_html": bottom

    });

    let s2 = state.template_service.render_template("htmx-test.html", args);
    // info!("s2 = {}", s2);
    Html::from(s2)
}


pub async fn hello(name: Query<Param>) -> String {
    format!("hello , {}", name.0.name).to_string()
}