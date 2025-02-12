use axum::{
    extract::{Path, Query, State},
    response::Response,
    Json,
};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::CampaignQuery,
    models::campaigns::{CampaignModel, CampaignRow, GenderModel},
    utils::paginate,
    AppState,
};

pub async fn route(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Query(query): Query<CampaignQuery>,
) -> Result<Json<Vec<CampaignModel>>, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let rows = sqlx::query_as!(
        CampaignRow,
        r#"
        SELECT id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
               cost_per_click, ad_title, ad_text, start_date, end_date,
               gender as "gender: GenderModel", age_from, age_to, location
        FROM campaigns
        WHERE advertiser_id = $1
        "#,
        advertiser_id
    )
    .fetch_all(&mut *conn)
    .await
    .map_err(ProdError::DatabaseError)?;

    if rows.is_empty() {
        return Err(ProdError::NotFound(format!(
            "No campaigns were found for advertiser - `{:?}`",
            advertiser_id
        ))
        .into());
    }

    let campaigns = paginate(rows, query.size, query.page)
        .into_iter()
        .map(CampaignModel::from)
        .collect();

    Ok(Json(campaigns))
}
