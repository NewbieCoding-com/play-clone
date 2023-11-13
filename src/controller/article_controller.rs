use std::sync::Arc;

use axum::extract::State;
use axum::response::Html;
use axum::Router;
use axum::routing::get;
use serde_json::json;

use crate::AppState;
use crate::controller::{R, S};

pub fn init() -> Router<Arc<AppState>> {
    Router::new()
        .route("/article", get(test))
}

async fn test() -> R<Html<&'static str>> {
    Ok(Html("ok333444."))
}
