use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use validator::Validate;

use crate::{
    db::Db,
    errors::ProdError,
    map_vec,
    models::clients::{Client, Gender},
    AppState,
};

pub async fn bulk(
    State(state): State<Arc<AppState>>,
    Json(clients): Json<Vec<Client>>,
) -> Result<(StatusCode, Json<Vec<Client>>), Response<String>> {
    clients.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;

    let _ = sqlx::query!(
        r#"
        INSERT INTO clients(id, login, age, location, gender)
        SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[], $3::INT[], $4::VARCHAR[], $5::GENDER[])
        "#,
        &map_vec!(clients, client_id),
        &map_vec!(clients, login),
        &map_vec!(clients, age),
        &map_vec!(clients, location),
        map_vec!(clients, gender) as Vec<Gender>,
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(ProdError::DatabaseError)?;

    Ok((StatusCode::CREATED, Json(clients)))
}

pub async fn get_client_by_id(
    State(state): State<Arc<AppState>>,
    Path(client_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<Client>), Response<String>> {
    let mut conn = state.pool.conn().await?;

    let client = sqlx::query_as!(
        Client,
        r#"
        SELECT id as client_id,
               login, age, location,
               gender as "gender: Gender"
        FROM clients
        WHERE id = $1
        LIMIT 1
        "#,
        client_id
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            ProdError::NotFound("No client was found with that id.".to_string())
        }
        err => ProdError::DatabaseError(err),
    })?;

    Ok((StatusCode::OK, Json(client)))
}
