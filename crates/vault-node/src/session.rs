use crate::{envelope::Envelope, state::AppState};
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::post,
};
use serde_json::json;

async fn start_session(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> StatusCode {
    let result = state.mailboxes.open(session_id.clone());

    let mut mailbox_reciever = match result {
        Some(e) => e,
        None => return StatusCode::CONFLICT,
    };

    tokio::spawn(async move {
        println!("worker started for session {session_id}");
        let hello = Envelope {
            broadcast: false,
            session_id: session_id.clone(),
            sender_id: state.node_index,
            payload: json!({
                "type": "hello",
                "node_index": state.node_index
            }),
        };

        for to in 0..3 {
            if to == state.node_index {
                continue;
            }
            let url = state.outbox.to_url(to);
            let sent = state.outbox.send(to, &hello).await;
            match sent {
                Ok(_) => println!("sent hello to {url}"),
                Err(e) => println!("failed to send hello to {url}: {e:?}"),
            };
        }

        loop {
            let next = mailbox_reciever.recv().await;
            match next {
                Some(envelope) => {
                    let from = state.outbox.to_url(envelope.sender_id);
                    println!("worker {session_id} got from {from}: {}", envelope.payload);
                }
                None => {
                    println!("worker {session_id} mailbox closed");
                    break;
                }
            }
        }
    });

    StatusCode::OK
}

pub fn router() -> Router<AppState> {
    Router::new().route("/session/{id}/start", post(start_session))
}
