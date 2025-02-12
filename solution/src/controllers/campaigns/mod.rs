mod routes;

use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};
use routes::{create, get_campaign_by_id, list};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(create))
        .route("/campaigns", get(list))
        .route("/campaigns/{compaign_id}", get(get_campaign_by_id))
}
