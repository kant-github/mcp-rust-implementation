use axum::Router;
use sea_orm::DatabaseConnection;
use tower_http::cors::CorsLayer;

use crate::{health, wallet};

pub fn app(db: DatabaseConnection, cors: CorsLayer) -> Router {
    Router::new()
        .merge(health::router())
        .nest("/v1", wallet::router().with_state(db))
        .layer(cors)
}
