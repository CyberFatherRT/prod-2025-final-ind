use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TimeModel {
    pub current_date: i32,
}
