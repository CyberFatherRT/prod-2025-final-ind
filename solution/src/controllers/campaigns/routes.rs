use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Response,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    db::Db,
    errors::ProdError,
    forms::campaigns::{CampaignForm, CampaignPatchForm, CampaignQuery, GenderForm},
    models::campaigns::{CampaignModel, CampaignRow, GenderModel},
    utils::paginate,
    AppState,
};

pub async fn create(
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

pub async fn list(
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

pub async fn get_campaign_by_id(
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

pub async fn update(
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

pub async fn delete_campaign(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, Response<String>> {
    let mut conn = state.pool.conn().await?;

    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM campaigns
        WHERE advertiser_id = $1 AND id = $2
        "#,
        advertiser_id,
        campaign_id
    )
    .execute(&mut *conn)
    .await
    .map_err(ProdError::DatabaseError)?;

    if rows_affected.rows_affected() == 0 {
        return Err(ProdError::NotFound(format!(
            "No campaign was found with id - `{:?}` for advertiser - `{:?}`",
            campaign_id, advertiser_id
        ))
        .into());
    }

    Ok(StatusCode::NO_CONTENT)
}
