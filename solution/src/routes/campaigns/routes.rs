use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::{CampaignForm, CampaignPatchForm, CampaignQuery},
    models::campaigns::CampaignController,
    models::campaigns::CampaignModel,
    AppState,
};

pub async fn create(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Json(campaign): Json<CampaignForm>,
) -> Result<(StatusCode, Json<CampaignModel>), Response<String>> {
    campaign.validate().map_err(ProdError::InvalidRequest)?;
    let mut conn = state.pool.conn().await?;
    let campaign = CampaignModel::create(&mut *conn, advertiser_id, campaign).await?;

    Ok((StatusCode::CREATED, Json(campaign)))
}

pub async fn list(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Query(query): Query<CampaignQuery>,
) -> Result<Json<Vec<CampaignModel>>, Response<String>> {
    let mut conn = state.pool.conn().await?;
    let campaigns = CampaignModel::list(&mut *conn, advertiser_id, query).await?;

    Ok(Json(campaigns))
}

pub async fn update(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
    Json(campaign): Json<CampaignPatchForm>,
) -> Result<Json<CampaignModel>, Response<String>> {
    let mut conn = state.pool.conn().await?;
    let campaing = CampaignModel::update(&mut *conn, advertiser_id, campaign_id, campaign).await?;

    let foo = campaing;
    Ok(Json(foo))
}

pub async fn get_campaign_by_id(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<CampaignModel>, Response<String>> {
    let mut conn = state.pool.conn().await?;
    let campaign = CampaignModel::get(&mut *conn, advertiser_id, campaign_id).await?;

    Ok(Json(campaign))
}

pub async fn delete_campaign(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, Response<String>> {
    let mut conn = state.pool.conn().await?;
    let _ = CampaignModel::delete(&mut *conn, advertiser_id, campaign_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
