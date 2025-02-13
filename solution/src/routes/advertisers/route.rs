use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
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

pub async fn bulk(
    State(state): State<AppState>,
    Json(advertisers): Json<Vec<AdvertiserForm>>,
) -> Result<(StatusCode, Json<Vec<AdvertiserModel>>), Response<String>> {
    advertisers.validate().map_err(ProdError::InvalidRequest)?;

    let mut conn = state.pool.conn().await?;
    let advertisers = AdvertiserModel::bulk(&mut conn, advertisers).await?;

    Ok((StatusCode::CREATED, Json(advertisers)))
}

pub async fn get_advertiser_by_id(
    State(state): State<AppState>,
    Path(advertiser_id): Path<uuid::Uuid>,
) -> Result<(StatusCode, Json<AdvertiserModel>), Response<String>> {
    let mut conn = state.pool.conn().await?;
    let advertiser = AdvertiserModel::get_advertiser_by_id(&mut conn, advertiser_id).await?;

    Ok((StatusCode::OK, Json(advertiser)))
}

pub async fn ml_scores(
    State(state): State<AppState>,
    Json(ml_score): Json<MlScoreForm>,
) -> Result<StatusCode, Response<String>> {
    let mut conn = state.pool.conn().await?;
    AdvertiserModel::ml_scores(&mut conn, ml_score).await?;

    Ok(StatusCode::OK)
}
