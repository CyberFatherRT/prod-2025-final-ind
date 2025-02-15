use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::models::advertisers::{AdvertiserModel, MlScoreModel};

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct AdvertiserForm {
    #[serde(rename = "advertiser_id")]
    pub id: Uuid,
    pub name: String,
}

#[derive(Validate, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct MlScoreForm {
    pub client_id: Uuid,
    pub advertiser_id: Uuid,
    pub score: i32,
}

impl From<&AdvertiserForm> for AdvertiserModel {
    fn from(value: &AdvertiserForm) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
        }
    }
}

impl From<&MlScoreForm> for MlScoreModel {
    fn from(value: &MlScoreForm) -> Self {
        Self {
            client_id: value.client_id,
            advertiser_id: value.advertiser_id,
            score: value.score,
        }
    }
}
