mod routes;

use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use routes::{create, delete_campaign, get_campaign_by_id, list, update};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(create))
        .route("/campaigns", get(list))
        .route("/campaigns/{compaign_id}", get(get_campaign_by_id))
        .route("/campaigns/{compaign_id}", put(update))
        .route("/campaigns/{compaign_id}", delete(delete_campaign))
}
