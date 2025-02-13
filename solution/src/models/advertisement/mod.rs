use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, Pool, Postgres};
use uuid::Uuid;

use crate::errors::ProdError;

#[derive(Serialize, Deserialize, FromRow)]
pub struct AdvertisementModel {
    #[serde(rename = "ad_id")]
    pub campaign_id: Uuid,
    pub ad_title: String,
    pub ad_text: String,
    pub advertiser_id: Uuid,
}

pub trait AdModelController {
    async fn get_best_ad(
        conn: Pool<Postgres>,
        client_id: Uuid,
        date: i32,
    ) -> Result<AdvertisementModel, ProdError>;

    async fn click_ad(
        conn: Pool<Postgres>,
        client_id: Uuid,
        campaign_id: Uuid,
        date: i32,
    ) -> Result<(), ProdError>;
}
