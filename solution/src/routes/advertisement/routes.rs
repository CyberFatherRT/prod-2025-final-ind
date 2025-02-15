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

/// Get advertisement for a client
#[utoipa::path(
    get,
    tag = "Ads",
    path = "/ads",
    responses(
        (status = 200, body = AdvertisementModel),
        (status = 404, description = "No advertisement found")
    ),
    params(
        ("client_id" = Uuid, Query, description = "Client ID"),
    )
)]
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

/// Recording a click on an ad
#[utoipa::path(
    post,
    tag = "Ads",
    path = "/ads/{campaign_id}/click",
    responses(
        (status = 200),
        (status = 404, description = "No advertisement found or client did not see the ad")
    ),
    params(
        ("campaign_id" = Uuid, Path, description = "Campaign ID"),
    )
)]
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
