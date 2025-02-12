use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Type)]
#[sqlx(type_name = "CAMPAIGN_GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum GenderForm {
    Male,
    Female,
    All,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TargetForm {
    pub gender: GenderForm,
    pub age_from: i32,
    pub age_to: i32,
    pub location: String,
}

impl Validate for TargetForm {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.age_from > self.age_to {
            return Err(validator::ValidationErrors::new());
        }
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CampaignForm {
    pub impressions_limit: i32,
    pub clicks_limit: i32,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i32,
    pub end_date: i32,
    pub targeting: TargetForm,
}

impl Validate for CampaignForm {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        self.targeting.validate()?;

        if self.start_date > self.end_date {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}
