use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{envelope::Envelope, state::AppState};

async fn recieve_message(
    State(state): State<AppState>,
    Json(envelope): Json<Envelope>,
) -> StatusCode {
    state.mailboxes.deliver(envelope);
    StatusCode::ACCEPTED
}

pub fn router() -> Router<AppState> {
    Router::new().route("/message", post(recieve_message))
}
