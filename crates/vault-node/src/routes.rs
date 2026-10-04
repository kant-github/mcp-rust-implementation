use axum::Router;
use crate::{health, message, session, state::AppState};

pub fn app(state: AppState) -> Router {
    Router::new().merge(health::router()).nest(
        "/v1",
        message::router()
            .merge(session::router())
            .with_state(state),
    )
}
