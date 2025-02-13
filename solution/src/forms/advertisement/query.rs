use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AdvertisementQuery {
    pub client_id: Uuid,
}
