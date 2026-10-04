mod envelope;
mod health;
mod mailbox;
mod message;
mod routes;
mod session;

#[tokio::main]
async fn main() {
    let config = config::load_node_config().expect("didnt found the right envs");
    let address = format!("0.0.0.0:{}", config.port);

    let app = routes::app(mailbox::MailBoxes::default());

    println!("Starting server on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
