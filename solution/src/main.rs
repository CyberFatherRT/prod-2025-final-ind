use axum::{
    http::StatusCode,
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use controllers::{advertisers, clients};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Level;
use utils::{env, log_request};

mod controllers;
mod db;
mod errors;
mod macros;
mod models;
mod utils;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
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

    let pool = PgPool::connect(&db_url).await?;

    sqlx::migrate!("./migrations").run(&pool).await?;
    let app_state = AppState { pool };

    let api = Router::new()
        .nest("/clients", clients::get_routes())
        .nest("/advertisers", advertisers::get_routes())
        .route("/health", get(StatusCode::OK))
        .route("/ml-scores", post(advertisers::ml_scores))
        .layer(from_fn(log_request))
        .with_state(app_state);

    let app = Router::new().nest("/api", api);

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
