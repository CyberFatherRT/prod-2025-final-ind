use std::fmt::Debug;

use sqlx::{Pool, Postgres};
use tracing::info;

use crate::{
    db::Db,
    errors::ProdError,
    models::advertisement::{AdModelController, AdvertisementModel},
};

impl AdModelController for AdvertisementModel {
    async fn get_best_ad(
        pool: Pool<Postgres>,
        client_id: uuid::Uuid,
        date: i32,
    ) -> Result<Self, ProdError> {
        let mut tx = pool.begin().await?;

        let row = sqlx::query!(
            r#"
            SELECT * FROM get_best_ad($1, $2)
            "#,
            client_id,
            date,
        )
        .fetch_one(&mut *tx)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => {
                ProdError::NotFound(format!("No advertisement found for client {client_id}"))
            }
            _ => ProdError::DatabaseError(err),
        })?;

        info!("get best ad row: {:?}", row);
        let advertisement = Self {
            campaign_id: row.campaign_id.expect("campaign_id"),
            ad_title: row.ad_title.expect("ad_name"),
            ad_text: row.ad_text.expect("ad_text"),
            advertiser_id: row.advertiser_id.expect("advertiser_id"),
        };

        let _ = sqlx::query!(
            r#"
            INSERT INTO ad_impressions(client_id, campaign_id, advertiser_id, impression_date)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT DO NOTHING
            "#,
            client_id,
            advertisement.campaign_id,
            advertisement.advertiser_id,
            date,
        )
        .execute(&mut *tx)
        .await
        .map_err(ProdError::DatabaseError)?;

        Ok(advertisement)
    }

    async fn click_ad(
        pool: Pool<Postgres>,
        client_id: uuid::Uuid,
        campaign_id: uuid::Uuid,
        date: i32,
    ) -> Result<(), ProdError> {
        let mut conn = pool.conn().await?;

        let _ = sqlx::query!(
            r#"
            INSERT INTO ad_clicks(client_id, campaign_id, advertiser_id, click_date)
            SELECT $1, $2, advertiser_id, $3 FROM campaigns WHERE id = $2
            ON CONFLICT DO NOTHING
            "#,
            client_id,
            campaign_id,
            date,
        )
        .execute(&mut *conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        Ok(())
    }
}
