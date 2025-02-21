use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::{CampaignForm, CampaignPatchForm, CampaignQuery},
    models::campaigns::{CampaignController, CampaignModel},
    AppState,
};

/// Create a new advertisement campaign
#[utoipa::path(
    post,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns",
    responses(
        (status = 201, body = CampaignModel),
        (status = 400,  description = "Invalid request")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
    )
)]
pub async fn create(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Json(campaign): Json<CampaignForm>,
) -> Result<(StatusCode, Json<CampaignModel>), ProdError> {
    campaign.validate().map_err(ProdError::InvalidRequest)?;

    // try_join!(
    //     llm_validate(&campaign.ad_title),
    //     llm_validate(&campaign.ad_text)
    // )?;

    let mut conn = state.pool.conn().await?;
    let campaign = CampaignModel::create(&mut conn, advertiser_id, campaign).await?;

    Ok((StatusCode::CREATED, Json(campaign)))
}

/// List all advertisement campaigns with pagination
#[utoipa::path(
    get,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns",
    responses(
        (status = 200, body = Vec<CampaignModel>),
        (status = 400,  description = "Invalid request"),
        (status = 404,  description = "Campaigns not found")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
        ("size" = usize, Query, description = "Number of items per page"),
        ("page" = usize, Query, description = "Page number"),
    )
)]
pub async fn list(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Query(query): Query<CampaignQuery>,
) -> Result<Json<Vec<CampaignModel>>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaigns = CampaignModel::list(&mut conn, advertiser_id, query).await?;

    Ok(Json(campaigns))
}

/// Update an advertisement campaign by ID
#[utoipa::path(
    put,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    responses(
        (status = 200, body = CampaignModel),
        (status = 400, description = "Invalid request"),
        (status = 404, description = "Campaign not found")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
        ("campaign_id" = uuid::Uuid, Path, description = "Campaign ID"),
    )
)]
pub async fn update(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
    Json(campaign): Json<CampaignPatchForm>,
) -> Result<Json<CampaignModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaing = CampaignModel::update(&mut conn, advertiser_id, campaign_id, campaign).await?;

    Ok(Json(campaing))
}

/// Get an advertisement campaign by ID
#[utoipa::path(
    get,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    responses(
        (status = 200, body = CampaignModel),
        (status = 400,  description = "Invalid request"),
        (status = 404,  description = "Campaign not found")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
        ("campaign_id" = uuid::Uuid, Path, description = "Campaign ID"),
    )
)]
pub async fn get_campaign_by_id(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<CampaignModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaign = CampaignModel::get(&mut conn, advertiser_id, campaign_id).await?;

    Ok(Json(campaign))
}

/// Delete an advertisement campaign by ID
#[utoipa::path(
    delete,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}",
    responses(
        (status = 204, body = ()),
        (status = 400,  description = "Invalid request"),
        (status = 404,  description = "Campaign not found")
    ),
    params(
        ("advertiser_id" = uuid::Uuid, Path, description = "Advertiser ID"),
        ("campaign_id" = uuid::Uuid, Path, description = "Campaign ID"),
    )
)]
pub async fn delete_campaign(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, ProdError> {
    let mut conn = state.pool.conn().await?;
    CampaignModel::delete(&mut conn, advertiser_id, campaign_id).await?;

    Ok(StatusCode::NO_CONTENT)
}
