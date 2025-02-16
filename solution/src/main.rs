#[cfg(debug_assertions)]
use solution::openapi::ApiDoc;
use solution::{app, services::s3::setup_s3, utils::env, AppState};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::Level;
#[cfg(debug_assertions)]
use utoipa::OpenApi;
#[cfg(debug_assertions)]
use utoipa_swagger_ui::SwaggerUi;

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
    let (s3, bucket_name) = setup_s3().await?;
    let app_state = AppState {
        pool,
        rclient,
        s3,
        bucket_name,
    };

    let app = app(app_state);

    #[cfg(debug_assertions)]
    let app = app
        .merge(SwaggerUi::new("/api/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    let addr = format!("0.0.0.0:{}", port);
    let listener = TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
