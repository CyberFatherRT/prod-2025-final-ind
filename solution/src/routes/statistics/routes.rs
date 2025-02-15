use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::ProdError,
    models::statistics::{
        DailyStatisticsModel, DailyStatisticsModelController, StatisticsModel,
        StatisticsModelController,
    },
    AppState,
};

/// Get campaign statistics
#[utoipa::path(
    get,
    tag = "Statistics",
    path = "/stats/campaigns/{campaign_id}",
    responses(
        (status = 200, body = StatisticsModel),
        (status = 404,  description = "Campaign not found")
    ),
    params(
        ("campaign_id" = Uuid, Path, description = "Campaign ID"),
    )
)]
pub async fn get_campaign_statistics(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
) -> Result<Json<StatisticsModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaign_statistics = StatisticsModel::campaign_statistics(&mut conn, campaign_id).await?;

    Ok(Json(campaign_statistics))
}

/// Get agregated statistics for all campaigns of an advertiser
#[utoipa::path(
    get,
    tag = "Statistics",
    path = "/stats/advertiser/{advertiser_id}/campaigns",
    responses(
        (status = 200, body = StatisticsModel),
        (status = 404,  description = "Advertiser not found")
    ),
    params(
        ("advertiser_id" = Uuid, Path, description = "Advertiser ID"),
    )
)]
pub async fn get_advertiser_statistics(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
) -> Result<Json<StatisticsModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let advertiser_statistics =
        StatisticsModel::advertiser_statistics(&mut conn, advertiser_id).await?;

    Ok(Json(advertiser_statistics))
}

/// Get daily statistics for a campaign
#[utoipa::path(
    get,
    tag = "Statistics",
    path = "/stats/campaigns/{campaign_id}/daily",
    responses(
        (status = 200, body = Vec<DailyStatisticsModel>),
        (status = 404,  description = "Campaign not found")
    ),
    params(
        ("campaign_id" = Uuid, Path, description = "Campaign ID"),
    )
)]
pub async fn get_campaign_daily_statistics(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
) -> Result<Json<Vec<DailyStatisticsModel>>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaign_daily_statistics =
        DailyStatisticsModel::campaign_daily_statistics(&mut conn, campaign_id).await?;

    Ok(Json(campaign_daily_statistics))
}

/// Get daily statistics for all campaigns of an advertiser
#[utoipa::path(
    get,
    tag = "Statistics",
    path = "/stats/advertiser/{advertiser_id}/campaigns/daily",
    responses(
        (status = 200, body = Vec<DailyStatisticsModel>),
        (status = 404,  description = "Advertiser not found")
    ),
    params(
        ("advertiser_id" = Uuid, Path, description = "Advertiser ID"),
    )
)]
pub async fn get_advertiser_daily_statistics(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
) -> Result<Json<Vec<DailyStatisticsModel>>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let advertiser_daily_statistics =
        DailyStatisticsModel::advertiser_daily_statistics(&mut conn, advertiser_id).await?;

    Ok(Json(advertiser_daily_statistics))
}
