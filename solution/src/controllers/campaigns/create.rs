use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::{CampaignForm, GenderForm},
    models::campaigns::{CampaignModel, CampaignRow, GenderModel},
    AppState,
};

pub async fn route(
    State(state): State<AppState>,
    Path(advertiser_id): Path<Uuid>,
    Json(campaign): Json<CampaignForm>,
) -> Result<(StatusCode, Json<CampaignModel>), Response<String>> {
    campaign.validate().map_err(ProdError::InvalidRequest)?;
    let mut conn = state.pool.conn().await?;

    let targeting = campaign.targeting.unwrap_or_default();

    let row = sqlx::query_as!(
        CampaignRow,
        r#"
        INSERT INTO campaigns(advertiser_id, impressions_limit, clicks_limit, cost_per_impression, cost_per_click, ad_title,
                     ad_text, start_date, end_date, gender, age_from, age_to, location)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
        RETURNING id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                  cost_per_click, ad_title, ad_text, start_date, end_date,
                  gender AS "gender: GenderModel",
                  age_from, age_to, location
        "#,
        advertiser_id,
        campaign.impressions_limit,
        campaign.clicks_limit,
        campaign.cost_per_impression,
        campaign.cost_per_click,
        campaign.ad_title,
        campaign.ad_text,
        campaign.start_date,
        campaign.end_date,
        targeting.gender as Option<GenderForm>,
        targeting.age_from,
        targeting.age_to,
        targeting.location
    )
    .fetch_one(&mut *conn)
    .await
    .map_err(|err| match err {
        sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
            ProdError::NotFound(format!("No advertiser was found with that id - `{:?}`", advertiser_id))
        },
        _ => ProdError::DatabaseError(err),
    })?;

    Ok((StatusCode::CREATED, Json(CampaignModel::from(row))))
}
