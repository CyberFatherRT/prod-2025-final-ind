use axum::{
    http::{header, Request},
    Router,
};
use serde_json::json;
use tower::{Service, ServiceExt};

pub async fn bulk(app: Router) -> anyhow::Result<()> {
    let mut app = app.into_service();

    let missing_client_id = json!([
        {
           "login": "client_1",
           "age": 20,
           "location": "Moscow",
           "gender": "MALE",
        },
    ]);

    let missing_login = json!([
        {
           "client_id": uuid::Uuid::new_v4(),
           "age": 20,
           "location": "Moscow",
           "gender": "MALE"
        },
    ]);

    let missing_age = json!([
        {
           "client_id": uuid::Uuid::new_v4(),
           "login": "client_1",
           "location": "Moscow",
           "gender": "MALE",
        },
    ]);

    let missing_location = json!([
        {
           "client_id": uuid::Uuid::new_v4(),
           "login": "client_2",
           "age": 45,
           "gender": "FEMALE"
        }
    ]);

    let missing_gender = json!([
        {
           "client_id": uuid::Uuid::new_v4(),
           "login": "client_2",
           "age": 45,
           "location": "Saint Petersburg"
        }
    ]);

    let bodys = vec![
        missing_client_id,
        missing_login,
        missing_age,
        missing_location,
        missing_gender,
    ];

    for body in bodys {
        let request = Request::builder()
            .uri("/api/clients/bulk")
            .method("POST")
            .header(header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(body.to_string())?;

        let response = ServiceExt::ready(&mut app).await?.call(request).await?;
        assert_eq!(response.status(), 400);
    }

    Ok(())
}

pub async fn get(app: Router) -> anyhow::Result<()> {
    let mut app = app.into_service();

    let request = Request::builder()
        .uri("/api/clients/")
        .method("GET")
        .body(String::new())?;

    let response = ServiceExt::ready(&mut app).await?.call(request).await?;
    assert_eq!(response.status(), 404);

    Ok(())
}
