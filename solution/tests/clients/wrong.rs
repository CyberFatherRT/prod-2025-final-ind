use axum::{http::method::Method, Router};
use serde_json::json;
use tower::{Service, ServiceExt};

pub async fn bulk(app: Router) -> anyhow::Result<()> {
    let mut app = app.into_service();

    let wrong_client_id = json!([{
       "client_id": "f0d3f7dd-c5c1-4742-not-uuid-2bd43df4d2d0",
       "login": "client_1",
       "age": 20,
       "location": "Moscow",
       "gender": "MALE"
    }]);

    let login_too_short = json!([{
       "client_id": uuid::Uuid::new_v4(),
       "login": "c",
       "age": 20,
       "location": "Moscow",
       "gender": "Male"
    }]);

    let wrong_age_min = json!([{
       "client_id": uuid::Uuid::new_v4(),
       "login": "client_1",
       "age": -5,
       "location": "Moscow",
       "gender": "MALE"
    }]);

    let wrong_location_min = json!([{
       "client_id": uuid::Uuid::new_v4(),
       "login": "client_1",
       "age": 20,
       "location": "",
       "gender": "MALE"
    }]);

    let wrong_gender = json!([{
       "client_id": uuid::Uuid::new_v4(),
       "login": "client_1",
       "age": 20,
       "location": "Moscow",
       "gender": "Male",
    }]);

    let bodies = vec![
        wrong_client_id,
        login_too_short,
        wrong_age_min,
        wrong_location_min,
        wrong_gender,
    ];

    for body in bodies {
        let request = axum::http::Request::builder()
            .uri("/clients/bulk")
            .method(Method::POST)
            .header(
                axum::http::header::CONTENT_TYPE,
                mime::APPLICATION_JSON.as_ref(),
            )
            .body(serde_json::to_string(&body)?)?;

        let response = ServiceExt::ready(&mut app).await?.call(request).await?;
        assert_eq!(response.status(), 400);
    }

    Ok(())
}
