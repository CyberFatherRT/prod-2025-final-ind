use axum::{http::StatusCode, routing::get, Router};
use controllers::clients::get_routes;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Level;
use utils::env;

pub mod controllers;
pub mod db;
pub mod errors;
pub mod macros;
pub mod models;
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

    let app = Router::new()
        .nest("/api/clients", get_routes(app_state))
        .route("/api/health", get(StatusCode::OK));

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}
