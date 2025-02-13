mod routes;

use axum::{routing::post, Router};
use routes::set_date;

use crate::AppState;

pub fn get_routes() -> Router<AppState> {
    Router::new().route("/advance", post(set_date))
}
