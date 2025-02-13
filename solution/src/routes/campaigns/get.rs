use axum::{
    extract::{Path, State},
    response::Response,
    Json,
};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::ProdError,
    models::campaigns::{CampaignModel, CampaignRow, GenderModel},
    AppState,
};

pub async fn route(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<Json<CampaignModel>, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let campaign = sqlx::query_as!(
        CampaignRow,
        r#"
        SELECT id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
               cost_per_click, ad_title, ad_text, start_date, end_date,
               gender as "gender: GenderModel", age_from, age_to, location
        FROM campaigns
        WHERE advertiser_id = $1 AND id = $2
        "#,
        advertiser_id,
        campaign_id
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => ProdError::NotFound(format!(
            "No campaign was found with id - `{:?}` for advertiser - `{:?}`",
            campaign_id, advertiser_id
        )),
        _ => ProdError::DatabaseError(err),
    })?;

    Ok(Json(campaign.into()))
}
