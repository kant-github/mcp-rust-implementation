use crate::state::AppState;

mod envelope;
mod health;
mod mailbox;
mod message;
mod outbox;
mod routes;
mod session;
mod state;

#[tokio::main]
async fn main() {
    let config = config::load_node_config().expect("didnt found the right envs");
    let address = format!("0.0.0.0:{}", config.port);
    let state = AppState {
        node_index: config.node_index,
        mailboxes: mailbox::MailBoxes::default(),
        outbox: outbox::Outbox::new(config.node_urls),
    };
    let app = routes::app(state);

    println!("Starting server on {}", address);
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, app).await.unwrap()
}
