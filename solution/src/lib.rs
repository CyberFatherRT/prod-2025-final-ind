#![deny(clippy::unwrap_used)]
#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(async_fn_in_trait)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::must_use_candidate)]

use axum::{
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use routes::{advertisement, advertisers, clients, statistics, time};
use sqlx::PgPool;
use utils::{change_status_code, log_request};

pub mod controllers;
pub mod db;
pub mod errors;
pub mod forms;
pub mod macros;
pub mod models;
#[cfg(debug_assertions)]
pub mod openapi;
pub mod routes;
pub mod services;
pub mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub rclient: redis::Client,
    pub s3: minio::s3::Client,
    pub bucket_name: String,
}

pub fn app(state: AppState) -> Router {
    Router::new()
        .route("/health", get(StatusCode::OK))
        .route("/ml-scores", post(advertisers::ml_scores))
        .nest("/clients", clients::get_routes())
        .nest("/advertisers", advertisers::get_routes())
        .nest("/ads", advertisement::get_routes())
        .nest("/stats", statistics::get_routes())
        .nest("/time", time::get_routes())
        .layer(from_fn(change_status_code))
        .layer(from_fn(log_request))
        .with_state(state)
}
