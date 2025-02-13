use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AdvertisementForm {
    pub client_id: Uuid,
}
