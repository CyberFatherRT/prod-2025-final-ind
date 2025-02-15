pub mod routes;

use axum::{
    routing::{get, post},
    Router,
};
use routes::{click_ad, get_ad};

use crate::AppState;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(get_ad))
        .route("/{campaign_id}/click", post(click_ad))
}
