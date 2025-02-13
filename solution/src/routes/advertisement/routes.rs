use axum::{
    extract::{Path, Query, State},
    response::Response,
    Json,
};

use crate::{
    forms::advertisement::{AdvertisementForm, AdvertisementPath, AdvertisementQuery},
    models::advertisement::{AdModelController, AdvertisementModel},
    AppState,
};

pub async fn get_ad(
    State(state): State<AppState>,
    Query(query): Query<AdvertisementQuery>,
) -> Result<Json<AdvertisementModel>, Response<String>> {
    // FIX: get current date from redis
    let date = 0;
    let AdvertisementQuery { client_id } = query;
    let advertisement = AdvertisementModel::get_best_ad(state.pool, client_id, date).await?;

    Ok(Json(advertisement))
}

pub async fn click_ad(
    State(state): State<AppState>,
    Path(path): Path<AdvertisementPath>,
    Json(body): Json<AdvertisementForm>,
) -> Result<(), Response<String>> {
    // FIX: get current date from redis
    let date = 0;
    let AdvertisementForm { client_id } = body;
    let AdvertisementPath { campaign_id } = path;

    AdvertisementModel::click_ad(state.pool, client_id, campaign_id, date).await?;

    Ok(())
}
