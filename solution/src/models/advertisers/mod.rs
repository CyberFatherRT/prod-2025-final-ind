use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct Advertiser {
    #[serde(rename = "advertiser_id")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow)]
pub struct MlScore {
    pub client_id: Uuid,
    pub advertiser_id: Uuid,
    pub score: i32,
}
