mod cors;
mod health;
mod routes;
mod wallet;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    let config = config::load_api_config()?;
    let address = format!("0.0.0.0:{}", config.port);

    let db = sea_orm::Database::connect(&config.database_url).await?;
    println!("connected to postgres");

    let app = routes::app(db, cors::layer(config)?);
    let listener = tokio::net::TcpListener::bind(&address).await?;

    println!("Starting server on {}", address);
    axum::serve(listener, app).await?;

    Ok(())
}
