use axum::Router;

use crate::{health, mailbox::MailBoxes, message};

pub fn app(mailboxes: MailBoxes) -> Router {
    Router::new().merge(health::router()).nest("/v1", message::router().with_state(mailboxes))
}

