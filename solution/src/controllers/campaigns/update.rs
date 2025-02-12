use axum::{
    extract::{Path, State},
    response::Response,
    Json,
};
use uuid::Uuid;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::{CampaignPatchForm, GenderForm},
    models::campaigns::{CampaignModel, CampaignRow, GenderModel},
    AppState,
};

pub async fn route(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
    Json(campaign): Json<CampaignPatchForm>,
) -> Result<Json<CampaignModel>, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let CampaignPatchForm {
        cost_per_click,
        ad_title,
        ad_text,
        targeting,
    } = campaign;

    let campaign = sqlx::query_as!(
        CampaignRow,
        r#"
        UPDATE campaigns
        SET cost_per_click = COALESCE($1, cost_per_click),
            ad_title = COALESCE($2, ad_title),
            ad_text = COALESCE($3, ad_text),
            gender = COALESCE($4, gender),
            age_from = COALESCE($5, age_from),
            age_to = COALESCE($6, age_to),
            location = COALESCE($7, location)
        WHERE advertiser_id = $8 AND id = $9
        RETURNING id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                  cost_per_click, ad_title, ad_text, start_date, end_date,
                  gender AS "gender: GenderModel",
                  age_from, age_to, location
        "#,
        cost_per_click,
        ad_title,
        ad_text,
        targeting.gender as Option<GenderForm>,
        targeting.age_from,
        targeting.age_to,
        targeting.location,
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
