use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{envelope::Envelope, mailbox::MailBoxes};

async fn recieve_message(
    State(mailboxes): State<MailBoxes>,
    Json(envelope): Json<Envelope>,
) -> StatusCode {
    println!("Recieved message: {:?}", envelope);
    println!("mailboxes : {:?}", mailboxes);
    mailboxes.deliver(envelope);
    StatusCode::ACCEPTED
}

pub fn router() -> Router<MailBoxes> {
    Router::new().route("/message", post(recieve_message))
}
