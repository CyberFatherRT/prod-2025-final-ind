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

pub async fn get_campaign_statistics(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
) -> Result<Json<StatisticsModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaign_statistics = StatisticsModel::campaign_statistics(&mut conn, campaign_id).await?;

    Ok(Json(campaign_statistics))
}

pub async fn get_advertiser_statistics(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
) -> Result<Json<StatisticsModel>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let advertiser_statistics =
        StatisticsModel::advertiser_statistics(&mut conn, advertiser_id).await?;

    Ok(Json(advertiser_statistics))
}

pub async fn get_campaign_daily_statistics(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
) -> Result<Json<Vec<DailyStatisticsModel>>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let campaign_daily_statistics =
        DailyStatisticsModel::campaign_daily_statistics(&mut conn, campaign_id).await?;

    Ok(Json(campaign_daily_statistics))
}

pub async fn get_advertiser_daily_statistics(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
) -> Result<Json<Vec<DailyStatisticsModel>>, ProdError> {
    let mut conn = state.pool.conn().await?;
    let advertiser_daily_statistics =
        DailyStatisticsModel::advertiser_daily_statistics(&mut conn, advertiser_id).await?;

    Ok(Json(advertiser_daily_statistics))
}
