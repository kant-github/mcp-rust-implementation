use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
    }))
}

pub fn router() -> Router {
    Router::new().route("/health", get(health_check))
}
