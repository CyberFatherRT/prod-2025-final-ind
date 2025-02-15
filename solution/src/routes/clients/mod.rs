pub mod routes;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use routes::{bulk, get_client_by_id};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/bulk", post(bulk))
        .route("/{client_id}", get(get_client_by_id))
}
