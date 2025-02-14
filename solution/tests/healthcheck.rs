use axum::{body::Body, http::Request};
use tower::ServiceExt;
mod setup;

#[tokio::test]
async fn healthcheck() -> anyhow::Result<()> {
    setup::init_tracing();

    let app = setup::get_app().await;
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await?;

    assert_eq!(response.status(), 200);

    Ok(())
}
