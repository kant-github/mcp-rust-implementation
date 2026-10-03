use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "message": "Service is running correctly"
    }))
}

pub fn router() -> Router {
    axum::Router::new().route("/health", get(health_check))
}
