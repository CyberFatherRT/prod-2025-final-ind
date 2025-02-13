mod create;
mod delete;
mod get;
mod list;
mod update;

use crate::AppState;
use axum::{
    routing::{delete, get, post, put},
    Router,
};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/campaigns", post(create::route))
        .route("/campaigns", get(list::route))
        .route("/campaigns/{compaign_id}", get(get::route))
        .route("/campaigns/{compaign_id}", put(update::route))
        .route("/campaigns/{compaign_id}", delete(delete::route))
}
