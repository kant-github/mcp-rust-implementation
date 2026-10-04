use axum::Router;
use crate::{health, mailbox::MailBoxes, message, session};

pub fn app(mailboxes: MailBoxes) -> Router {
    Router::new().merge(health::router()).nest(
        "/v1",
        message::router()
            .merge(session::router())
            .with_state(mailboxes),
    )
}
