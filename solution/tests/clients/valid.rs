use axum::{
    http::{header, Request},
    Router,
};
use http_body_util::BodyExt;
use serde_json::{json, Value};
use tower::{Service, ServiceExt};

pub async fn bulk(app: Router) -> anyhow::Result<()> {
    let mut app = app.into_service();

    let request_body = json!([
        {
           "client_id": "32f22d72-0a89-4140-855f-c22830c3775e",
           "login": "client_1",
           "age": 20,
           "location": "Moscow",
           "gender": "MALE"
        },
        {
           "client_id": "f0d3f7dd-c5c1-4742-b442-2bd43df4d2d0",
           "login": "client_2",
           "age": 42,
           "location": "Saint Petersburg",
           "gender": "FEMALE"
        },
    ]);

    let request = Request::builder()
        .uri("/api/clients/bulk")
        .method("POST")
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(request_body.to_string())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;
    assert_eq!(response.status(), 201);

    let body = response.into_body().collect().await?.to_bytes();
    let response_body: Value = serde_json::from_slice(&body)?;

    assert_eq!(response_body, request_body);

    let request = Request::builder()
        .uri("/api/clients/bulk")
        .method("POST")
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(json!([]).to_string())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;
    assert_eq!(response.status(), 201);

    let body = response.into_body().collect().await?.to_bytes();
    let response_body: Value = serde_json::from_slice(&body)?;
    assert_eq!(response_body, json!([]));

    let request_body = json!([
        {
           "client_id": "f0d3f7dd-c5c1-4742-b442-2bd43df4d2d0",
           "login": "client_2",
           "age": 45,
           "location": "Florida",
           "gender": "FEMALE"
        }
    ]);

    let request = Request::builder()
        .uri("/api/clients/bulk")
        .method("POST")
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(request_body.to_string())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;

    assert_eq!(response.status(), 201);

    let body = response.into_body().collect().await?.to_bytes();
    let response_body: Value = serde_json::from_slice(&body)?;
    assert_eq!(response_body, request_body);

    Ok(())
}

pub async fn get(app: Router) -> anyhow::Result<()> {
    let mut app = app.into_service();

    let request_body = json!([
        {
           "client_id": "8b064e99-6e04-4db9-9b94-1d182e318042",
           "login": "client_3",
           "age": 20,
           "location": "Moscow",
           "gender": "MALE",
        }
    ]);

    let request = Request::builder()
        .uri("/api/clients/bulk")
        .method("POST")
        .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
        .body(request_body.to_string())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;
    assert_eq!(response.status(), 201);

    let body = response.into_body().collect().await?.to_bytes();
    let response_body: Value = serde_json::from_slice(&body)?;

    assert_eq!(response_body, request_body);

    let request = Request::builder()
        .uri("/api/clients/8b064e99-6e04-4db9-9b94-1d182e318042")
        .method("GET")
        .body("".to_string())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await?.to_bytes();
    let response_body: Value = serde_json::from_slice(&body)?;
    assert_eq!(
        response_body,
        json!({
            "client_id": "8b064e99-6e04-4db9-9b94-1d182e318042",
            "login": "client_3",
            "age": 20,
            "location": "Moscow",
            "gender": "MALE",
        })
    );

    Ok(())
}
