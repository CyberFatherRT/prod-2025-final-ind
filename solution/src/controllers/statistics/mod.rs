use sqlx::PgConnection;
use tracing::info;
use uuid::Uuid;

use crate::{
    errors::ProdError,
    models::statistics::{
        DailyStatisticsModel, DailyStatisticsModelController, StatisticsModel,
        StatisticsModelController,
    },
};

impl StatisticsModelController for StatisticsModel {
    async fn campaign_statistics(
        conn: &mut sqlx::PgConnection,
        campaign_id: Uuid,
    ) -> Result<Self, ProdError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM get_ad_stats($1)
            "#,
            campaign_id
        )
        .fetch_one(conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        info!("campaign statistics row: {:?}", row);
        let campaign_statistics = Self {
            impressions_count: row.impressions_count.expect("impressions_count"),
            clicks_count: row.clicks_count.expect("clicks_count"),
            conversion: row.conversion.expect("conversion"),
            spent_impressions: row.spent_impressions.expect("spent_impressions"),
            spent_clicks: row.spent_clicks.expect("spent_clicks"),
            spent_total: row.spent_total.expect("spent_total"),
        };

        Ok(campaign_statistics)
    }

    async fn advertiser_statistics(
        conn: &mut sqlx::PgConnection,
        advertiser_id: Uuid,
    ) -> Result<Self, ProdError> {
        let row = sqlx::query!(
            r#"
            SELECT * FROM get_advertiser_stats($1)
            "#,
            advertiser_id
        )
        .fetch_one(conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        info!("advertiser statistics row: {:?}", row);
        let advertiser_statistics = Self {
            impressions_count: row.impressions_count.expect("impressions_count"),
            clicks_count: row.clicks_count.expect("clicks_count"),
            conversion: row.conversion.expect("conversion"),
            spent_impressions: row.spent_impressions.expect("spent_impressions"),
            spent_clicks: row.spent_clicks.expect("spent_clicks"),
            spent_total: row.spent_total.expect("spent_total"),
        };

        Ok(advertiser_statistics)
    }
}

impl DailyStatisticsModelController for DailyStatisticsModel {
    async fn campaign_daily_statistics(
        conn: &mut PgConnection,
        campaign_id: Uuid,
    ) -> Result<Vec<Self>, ProdError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM get_daily_stats_campaign($1)
            "#,
            campaign_id
        )
        .fetch_all(conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        info!("campaign daily statistics rows: {:?}", rows);
        let daily_statistics_campaings = rows
            .into_iter()
            .map(|row| Self {
                impressions_count: row.impressions_count.expect("impressions_count"),
                clicks_count: row.clicks_count.expect("clicks_count"),
                conversion: row.conversion.expect("conversion"),
                spent_impressions: row.spent_impressions.expect("spent_impressions"),
                spent_clicks: row.spent_clicks.expect("spent_clicks"),
                spent_total: row.spent_total.expect("spent_total"),
                date: row.date.expect("date"),
            })
            .collect();

        Ok(daily_statistics_campaings)
    }

    async fn advertiser_daily_statistics(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
    ) -> Result<Vec<Self>, ProdError> {
        let rows = sqlx::query!(
            r#"
            SELECT * FROM get_daily_stats_advertiser($1)
            "#,
            advertiser_id
        )
        .fetch_all(conn)
        .await
        .map_err(ProdError::DatabaseError)?;

        info!("advertiser daily statistics rows: {:?}", rows);
        let daily_statistics_advertisers = rows
            .into_iter()
            .map(|row| Self {
                impressions_count: row.impressions_count.expect("impressions_count"),
                clicks_count: row.clicks_count.expect("clicks_count"),
                conversion: row.conversion.expect("conversion"),
                spent_impressions: row.spent_impressions.expect("spent_impressions"),
                spent_clicks: row.spent_clicks.expect("spent_clicks"),
                spent_total: row.spent_total.expect("spent_total"),
                date: row.date.expect("date"),
            })
            .collect();

        Ok(daily_statistics_advertisers)
    }
}
