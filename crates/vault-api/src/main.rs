use axum::{Json, routing::get};
use serde_json::{Value, json};

fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
    }))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let config = config::load_api_config()?;
    let address = format!("0.0.0.0:{}", config.port);

    let _db = sea_orm::Database::connect(&config.database_url).await?;
    println!("connected to postgres");
    

    let app = axum::Router::new().route("/health", get(health_check()));
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Starting server on {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}
