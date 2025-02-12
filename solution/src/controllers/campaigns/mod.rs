mod routes;

use crate::AppState;
use axum::{routing::post, Router};
use routes::create;

pub fn get_routes() -> Router<AppState> {
    Router::new().route("/campaigns", post(create))
}
