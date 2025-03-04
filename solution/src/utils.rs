use std::time::Instant;

use axum::{extract::Request, http::StatusCode, response::Response};
use tracing::info;

pub fn env(key: &str) -> String {
    dotenvy::var(key).unwrap_or_else(|_| panic!("`{key}` environment variable not found"))
}

pub async fn log_request(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().clone();

    let response = next.run(req).await;

    let status = response.status();
    let latency = start.elapsed();

    info!(
        target: "solution",
        method = %method,
        path = %path,
        status = status.as_u16(),
        latency = ?latency,
        "request"
    );

    Ok(response)
}

pub async fn change_status_code(
    req: Request,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let mut response = next.run(req).await;

    if response.status() == StatusCode::UNPROCESSABLE_ENTITY {
        *response.status_mut() = StatusCode::BAD_REQUEST;
    }

    Ok(response)
}

pub fn paginate<T: Clone>(items: Vec<T>, size: Option<usize>, page: Option<usize>) -> Vec<T> {
    let total_items = items.len();
    let size = size.unwrap_or(total_items);
    let page = page.unwrap_or(1);

    if page == 0 {
        return Vec::new();
    }

    let start = (page - 1) * size;

    if start >= total_items {
        return Vec::new();
    }

    items.into_iter().skip(start).take(size).collect()
}
