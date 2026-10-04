use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};

use crate::mailbox::MailBoxes;

async fn start_session(
    State(mailboxes): State<MailBoxes>,
    Path(session_id): Path<String>,
) -> StatusCode {
    let result = mailboxes.open(session_id.clone());

    let mut mailbox_reciever = match result {
        Some(e) => e,
        None => return StatusCode::CONFLICT,
    };

    tokio::spawn(async move {
        println!("worker started for session {session_id}");
        loop {
            let next = mailbox_reciever.recv().await;
            match next {
                Some(envelope) => println!("worker {session_id} got: {envelope:?}"),
                None => {
                    println!("worker {session_id} closed");
                    break;
                }
            }
        }
    });

    StatusCode::OK
}

pub fn router() -> Router<MailBoxes> {
    Router::new().route("/session/{id}/start", post(start_session))
}
