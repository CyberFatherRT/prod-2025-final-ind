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
    models::advertisers::{Advertiser, MlScore},
    AppState,
};

pub async fn bulk(
    State(state): State<AppState>,
    Json(clients): Json<Vec<Advertiser>>,
) -> Result<(StatusCode, Json<Vec<Advertiser>>), Response<String>> {
    clients.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;

    let _ = sqlx::query!(
        r#"
        INSERT INTO advertisers(id, name)
        SELECT * FROM UNNEST($1::UUID[], $2::VARCHAR[])
        "#,
        &map_vec!(clients, id),
        &map_vec!(clients, name),
    )
    .execute(&mut *conn)
    .await
    .map_err(ProdError::DatabaseError)?;

    Ok((StatusCode::CREATED, Json(clients)))
}

pub async fn get_advertiser_by_id(
    State(state): State<AppState>,
    Path(advertiser_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<Advertiser>), Response<String>> {
    let mut conn = state.pool.conn().await?;

    let advertiser = sqlx::query_as!(
        Advertiser,
        r#"
        SELECT id, name FROM advertisers
        WHERE id = $1
        "#,
        advertiser_id
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => {
            ProdError::NotFound("No advertiser was found with that id.".to_string())
        }
        err => ProdError::DatabaseError(err),
    })?;

    Ok((StatusCode::OK, Json(advertiser)))
}

pub async fn ml_scores(
    State(state): State<AppState>,
    Json(ml_score): Json<MlScore>,
) -> Result<StatusCode, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let _ = sqlx::query!(
        r#"
        INSERT INTO ml_scores(client_id, advertiser_id, score)
        VALUES ($1, $2, $3)
        ON CONFLICT (client_id, advertiser_id) DO UPDATE SET score = excluded.score
        "#,
        ml_score.client_id,
        ml_score.advertiser_id,
        ml_score.score
    )
    .execute(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
            ProdError::NotFound("No client or advertiser was found with these ids".to_string())
        }
        _ => ProdError::DatabaseError(err),
    })?;

    Ok(StatusCode::OK)
}
