use axum::{Json, Router, extract::State, routing::get};
use serde_json::{Value, json};

#[derive(Clone)]
struct AppState {
    node_id: String,
}

async fn health_check(State(state): State<AppState>) -> Json<Value> {
    Json(json!({
        "status": "ok",
        "node_id": state.node_id,
    }))
}

#[tokio::main]
async fn main() {
    let config = config::load_node_config().expect("didnt found the right envs");
    let address = format!("0.0.0.0:{}", config.port);

    let app = Router::new()
        .route("/health", get(health_check))
        .with_state(AppState {
            node_id: config.node_id,
        });

    println!("Starting server on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
