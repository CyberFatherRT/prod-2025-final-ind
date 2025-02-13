use serde::{Deserialize, Serialize};
use sqlx::PgConnection;
use uuid::Uuid;

use crate::errors::ProdError;

#[derive(Serialize, Deserialize)]
pub struct StatisticsModel {
    pub impressions_count: i32,
    pub clicks_count: i32,
    pub conversion: f64,
    pub spent_impressions: f64,
    pub spent_clicks: f64,
    pub spent_total: f64,
}

#[derive(Serialize, Deserialize)]
pub struct DailyStatisticsModel {
    pub impressions_count: i32,
    pub clicks_count: i32,
    pub conversion: f64,
    pub spent_impressions: f64,
    pub spent_clicks: f64,
    pub spent_total: f64,
    pub date: i32,
}

pub trait StatisticsModelController {
    async fn campaign_statistics(
        conn: &mut PgConnection,
        campaign_id: Uuid,
    ) -> Result<StatisticsModel, ProdError>;

    async fn advertiser_statistics(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
    ) -> Result<StatisticsModel, ProdError>;
}

pub trait DailyStatisticsModelController {
    async fn campaign_daily_statistics(
        conn: &mut PgConnection,
        campaign_id: Uuid,
    ) -> Result<Vec<DailyStatisticsModel>, ProdError>;

    async fn advertiser_daily_statistics(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
    ) -> Result<Vec<DailyStatisticsModel>, ProdError>;
}
