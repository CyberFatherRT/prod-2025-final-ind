pub mod routes;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
pub use routes::ml_scores;
use routes::{bulk, get_advertiser_by_id};

use super::campaigns;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/bulk", post(bulk))
        .route("/{advertiser_id}", get(get_advertiser_by_id))
        .nest("/{adveriser_id}/campaigns", campaigns::get_routes())
}
