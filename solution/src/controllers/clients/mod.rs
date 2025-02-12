mod routes;

use std::sync::Arc;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use routes::{bulk, get_client_by_id};

pub fn get_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/bulk", post(bulk))
        .route("/{client_id}", get(get_client_by_id))
        .with_state(Arc::new(app_state))
}
