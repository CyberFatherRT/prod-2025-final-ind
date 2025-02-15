use axum::{
    extract::{Path, Query, State},
    Json,
};
use redis::AsyncCommands;
use uuid::Uuid;

use crate::{
    db::Rclient,
    errors::ProdError,
    forms::advertisement::{AdvertisementForm, AdvertisementQuery},
    models::advertisement::{AdModelController, AdvertisementModel},
    AppState,
};

pub async fn get_ad(
    State(state): State<AppState>,
    Query(query): Query<AdvertisementQuery>,
) -> Result<Json<AdvertisementModel>, ProdError> {
    let mut rclient = state.rclient.conn().await?;
    let date = rclient.get("date").await.unwrap_or(0);
    let AdvertisementQuery { client_id } = query;
    let advertisement = AdvertisementModel::get_best_ad(state.pool, client_id, date).await?;

    Ok(Json(advertisement))
}

pub async fn click_ad(
    State(state): State<AppState>,
    Path(campaign_id): Path<Uuid>,
    Json(body): Json<AdvertisementForm>,
) -> Result<(), ProdError> {
    let mut rclient = state.rclient.conn().await?;
    let date = rclient.get("date").await.unwrap_or(0);
    let AdvertisementForm { client_id } = body;

    AdvertisementModel::click_ad(state.pool, client_id, campaign_id, date).await?;

    Ok(())
}
