use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::AppState;

pub mod routes;
use routes::{create, delete_campaign, get_campaign_by_id, list, update};

use super::minio_s3::routes::{delete_file, download_file, upload_file};

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create))
        .route("/", get(list))
        .route("/{compaign_id}", get(get_campaign_by_id))
        .route("/{compaign_id}", put(update))
        .route("/{compaign_id}", delete(delete_campaign))
        .route("/{compaign_id}/file", post(upload_file))
        .route("/{compaign_id}/file/{file_name}", get(download_file))
        .route("/{compaign_id}/file/{file_name}", delete(delete_file))
}
