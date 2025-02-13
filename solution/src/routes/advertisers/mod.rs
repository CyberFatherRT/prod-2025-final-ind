mod route;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
pub use route::ml_scores;
use route::{bulk, get_advertiser_by_id};

use super::campaigns;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/bulk", post(bulk))
        .route("/{advertiser_id}", get(get_advertiser_by_id))
        .nest("/{adveriser_id}/", campaigns::get_routes())
}
