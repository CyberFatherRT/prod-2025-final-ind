use solution::{app, services::setup_s3, utils::env, AppState};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Level;

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
    let s3 = setup_s3().await;
    let app_state = AppState { pool, rclient, s3 };

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app(app_state)).await?;

    Ok(())
}
