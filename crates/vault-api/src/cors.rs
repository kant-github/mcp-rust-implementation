use axum::http::{HeaderValue, Method, header};
use config::ApiConfig;
use tower_http::cors::CorsLayer;

pub fn layer(config: ApiConfig) -> anyhow::Result<CorsLayer> {
    let origins = config
        .cors_origins
        .iter()
        .map(|origin| HeaderValue::from_str(origin))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE]))
}
