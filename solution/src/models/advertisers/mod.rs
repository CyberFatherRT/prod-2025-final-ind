use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct AdvertiserModel {
    #[serde(rename = "advertiser_id")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct MlScoreModel {
    pub client_id: Uuid,
    pub advertiser_id: Uuid,
    pub score: i32,
}
