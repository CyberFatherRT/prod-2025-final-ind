use axum::{
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use routes::{advertisement, advertisers, clients, statistics, time};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Level;
use utils::{env, log_request};

mod controllers;
mod db;
mod errors;
mod forms;
mod macros;
mod models;
mod routes;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub rclient: redis::Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_max_level(Level::DEBUG)
        .init();

    let port = env("PORT");
    let db_url = env("DATABASE_URL");
    let redis_url = env("REDIS_URL");

    let pool = PgPool::connect(&db_url).await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    let rclient = redis::Client::open(redis_url)?;
    let app_state = AppState { pool, rclient };

    let api = Router::new()
        .route("/health", get(StatusCode::OK))
        .route("/ml-scores", post(advertisers::ml_scores))
        .nest("/clients", clients::get_routes())
        .nest("/advertisers", advertisers::get_routes())
        .nest("/ads", advertisement::get_routes())
        .nest("/stats", statistics::get_routes())
        .nest("/time", time::get_routes())
        .layer(from_fn(log_request))
        .with_state(app_state);

    let app = Router::new().nest("/api", api);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
