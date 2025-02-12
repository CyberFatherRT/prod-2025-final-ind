use std::time::Instant;

use axum::{extract::Request, http::StatusCode, response::Response};
use tracing::info;

pub fn env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("`{}` environment variable not found", key))
}

pub async fn log_request(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    let res = next.run(req).await;

    let status = res.status();
    let latency = start.elapsed();

    info!(
        target: "solution",
        method = %method,
        path = %path,
        status = status.as_u16(),
        latency = ?latency,
        "request"
    );

    Ok(res)
}
