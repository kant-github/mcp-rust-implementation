use axum::{Json, Router, extract::State, http::StatusCode, routing::get};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::wallet::{model::Model, service};

async fn list_wallets(db: State<DatabaseConnection>) -> Result<Json<Vec<Model>>, StatusCode> {
    let result = service::list(&db).await;
    match result {
        Ok(wallets) => Ok(Json(wallets)),
        Err(err) => {
            eprintln!("list wallets failed: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

#[derive(Deserialize)]
struct CreateWalletRequest {
    name: String,
}

async fn create_wallet(
    db: State<DatabaseConnection>,
    Json(body): Json<CreateWalletRequest>,
) -> Result<Json<Model>, StatusCode> {
    let name = body.name.trim();

    if name.len() == 0 {
        return Err(StatusCode::BAD_REQUEST);
    } else if name.len() > 64 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let result = service::create(&db, body.name).await;
    match result {
        Ok(wallet) => Ok(Json(wallet)),
        Err(err) => {
            eprintln!("create wallet failed: {err:?}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }
}

pub fn router() -> Router<DatabaseConnection> {
    Router::new().route("/wallet", get(list_wallets).post(create_wallet))
}
