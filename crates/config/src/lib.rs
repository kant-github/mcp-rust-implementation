use anyhow::{Result, bail};
use std::env;

pub struct NodeConfig {
    pub port: String,
    pub node_id: String,
}

pub struct ApiConfig {
    pub port: String,
    pub node_urls: Vec<String>,
    pub database_url: String,
    pub cors_origins: Vec<String>,
}

fn get(key: &str, default: &str) -> String {
    let value_from_env = env::var(key);
    match value_from_env {
        Ok(value) if !value.trim().is_empty() => value.trim().to_string(),
        _ => default.to_string(),
    }
}

fn split_parts(s: &str) -> Vec<String> {
    s.split(",")
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(String::from)
        .collect()
}

pub fn load_node_config() -> Result<NodeConfig> {
    let node_id = get("NODE_ID", "");
    if node_id.is_empty() {
        bail!("NODE_ID is required");
    }

    Ok(NodeConfig {
        node_id,
        port: get("PORT", "4000"),
    })
}

pub fn load_api_config() -> Result<ApiConfig> {
    let node_urls = split_parts(&get("NODE_URLS", ""));

    if node_urls.is_empty() {
        bail!("NODE_URLS is required");
    }

    let database_url = get("DATABASE_URL", "");
    if database_url.is_empty() {
        bail!("DATABASE_URL is required");
    }

    let cors_origins = split_parts(&get("CORS_ORIGINS", ""));
    if cors_origins.is_empty() {
        bail!("CORS_ORIGINS is required");
    }

    Ok(ApiConfig {
        port: get("PORT", "4000"),
        node_urls,
        database_url,
        cors_origins,
    })
}
