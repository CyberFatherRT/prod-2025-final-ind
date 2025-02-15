use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use validator::Validate;

use crate::{
    controllers::advertisers::AdvertiserContoller,
    db::Db,
    errors::ProdError,
    forms::advertisers::{AdvertiserForm, MlScoreForm},
    models::advertisers::AdvertiserModel,
    AppState,
};

/// Bulk insert/update of advertisers
#[utoipa::path(
    post,
    tag = "Advertisers",
    path = "/advertisers/bulk",
    responses(
        (status = 201, body = Vec<AdvertiserModel>),
        (status = 400,  description = "Invalid request")
    ),
)]
pub async fn bulk(
    State(state): State<AppState>,
    Json(advertisers): Json<Vec<AdvertiserForm>>,
) -> Result<(StatusCode, Json<Vec<AdvertiserModel>>), ProdError> {
    advertisers.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;
    let advertisers = AdvertiserModel::bulk(&mut conn, advertisers).await?;

    Ok((StatusCode::CREATED, Json(advertisers)))
}

/// Get advertiser by ID
#[utoipa::path(
    get,
    tag = "Advertisers",
    path = "/advertisers/{advertiser_id}",
    responses(
        (status = 201, body = AdvertiserModel),
        (status = 400,  description = "Invalid request")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
    )
)]
pub async fn get_advertiser_by_id(
    State(state): State<AppState>,
    Path(advertiser_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<AdvertiserModel>), ProdError> {
    let mut conn = state.pool.conn().await?;
    let advertiser = AdvertiserModel::get_advertiser_by_id(&mut conn, advertiser_id).await?;

    Ok((StatusCode::OK, Json(advertiser)))
}

/// Add or update ML score
#[utoipa::path(
    post,
    tag = "Advertisers",
    path = "/ml_scores",
    responses(
        (status = 200, body = ()),
        (status = 400,  description = "Invalid request")
    ),
)]
pub async fn ml_scores(
    State(state): State<AppState>,
    Json(ml_score): Json<MlScoreForm>,
) -> Result<StatusCode, ProdError> {
    let mut conn = state.pool.conn().await?;
    AdvertiserModel::ml_scores(&mut conn, ml_score).await?;

    Ok(StatusCode::OK)
}
