mod routes;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use routes::{create, list};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(create))
        .route("/campaigns", get(list))
}
