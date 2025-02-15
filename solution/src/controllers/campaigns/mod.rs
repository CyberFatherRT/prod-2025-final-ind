use sqlx::PgConnection;
use uuid::Uuid;

use crate::{
    errors::ProdError,
    forms::campaigns::{CampaignForm, CampaignGenderForm, CampaignPatchForm, CampaignQuery},
    models::campaigns::{CampaignController, CampaignGenderModel, CampaignModel, CampaignRow},
    utils::paginate,
};

impl CampaignController for CampaignModel {
    async fn create(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign: CampaignForm,
    ) -> Result<Self, ProdError> {
        let targeting = campaign.targeting.unwrap_or_default();

        let row = sqlx::query_as!(
            CampaignRow,
            r#"
            INSERT INTO campaigns(advertiser_id, impressions_limit, clicks_limit, cost_per_impression, cost_per_click, ad_title,
                         ad_text, start_date, end_date, gender, age_from, age_to, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            RETURNING id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                      cost_per_click, ad_title, ad_text, start_date, end_date,
                      gender AS "gender: CampaignGenderModel",
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
            targeting.gender as Option<CampaignGenderForm>,
            targeting.age_from,
            targeting.age_to,
            targeting.location
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::Database(err) if err.is_foreign_key_violation() => {
                ProdError::NotFound(format!("No advertiser was found with that id - `{advertiser_id:?}`"))
            },
            _ => ProdError::DatabaseError(err),
        })?;

        Ok(row.into())
    }

    async fn list(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        query: CampaignQuery,
    ) -> Result<Vec<Self>, ProdError> {
        let rows = sqlx::query_as!(
            CampaignRow,
            r#"
            SELECT id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                   cost_per_click, ad_title, ad_text, start_date, end_date,
                   gender as "gender: CampaignGenderModel", age_from, age_to, location
            FROM campaigns
            WHERE advertiser_id = $1 AND is_deleted = false
            "#,
            advertiser_id
        )
        .fetch_all(&mut *conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        if rows.is_empty() {
            return Err(ProdError::NotFound(format!(
                "No campaigns were found for advertiser - `{advertiser_id:?}`",
            )));
        }

        let campaigns = paginate(rows, query.size, query.page)
            .into_iter()
            .map(Self::from)
            .collect();

        Ok(campaigns)
    }

    async fn update(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
        campaign: CampaignPatchForm,
    ) -> Result<Self, ProdError> {
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
            WHERE advertiser_id = $8 AND id = $9 AND is_deleted = false
            RETURNING id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                      cost_per_click, ad_title, ad_text, start_date, end_date,
                      gender AS "gender: CampaignGenderModel",
                      age_from, age_to, location
            "#,
            cost_per_click,
            ad_title,
            ad_text,
            targeting.gender as Option<CampaignGenderForm>,
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
                "No campaign was found with id - `{campaign_id:?}` for advertiser - `{advertiser_id:?}`",
            )),
            _ => ProdError::DatabaseError(err),
        })?;

        Ok(campaign.into())
    }

    async fn get(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
    ) -> Result<Self, ProdError> {
        let campaign = sqlx::query_as!(
            CampaignRow,
            r#"
            SELECT id, advertiser_id, impressions_limit, clicks_limit, cost_per_impression,
                   cost_per_click, ad_title, ad_text, start_date, end_date,
                   gender as "gender: CampaignGenderModel", age_from, age_to, location
            FROM campaigns
            WHERE advertiser_id = $1 AND id = $2 AND is_deleted = false
            "#,
            advertiser_id,
            campaign_id
        )
        .fetch_one(&mut *conn)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ProdError::NotFound(format!(
                "No campaign was found with id - `{campaign_id:?}` for advertiser - `{advertiser_id:?}`",
            )),
            _ => ProdError::DatabaseError(err),
        })?;

        Ok(campaign.into())
    }

    async fn delete(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
    ) -> Result<(), ProdError> {
        let rows_affected = sqlx::query!(
            r#"
            UPDATE campaigns
            SET is_deleted = true
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
                "No campaign was found with id - `{campaign_id:?}` for advertiser - `{advertiser_id:?}`",
            )));
        }

        Ok(())
    }
}
