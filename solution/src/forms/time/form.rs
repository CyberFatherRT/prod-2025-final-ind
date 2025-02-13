use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TimeForm {
    pub current_date: i64,
}
