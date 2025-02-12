use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CampaignQuery {
    pub size: Option<usize>,
    pub page: Option<usize>,
}
