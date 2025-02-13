use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AdvertisementPath {
    pub campaign_id: Uuid,
}
