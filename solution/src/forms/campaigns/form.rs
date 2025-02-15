use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Type, ToSchema)]
#[sqlx(type_name = "CAMPAIGN_GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum CampaignGenderForm {
    Male,
    Female,
    All,
}

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub struct TargetForm {
    pub gender: Option<CampaignGenderForm>,
    pub age_from: Option<i32>,
    pub age_to: Option<i32>,
    pub location: Option<String>,
}

impl Validate for TargetForm {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if self.age_from > self.age_to {
            return Err(validator::ValidationErrors::new());
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CampaignForm {
    pub impressions_limit: i32,
    pub clicks_limit: i32,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i32,
    pub end_date: i32,
    pub targeting: Option<TargetForm>,
}

impl Validate for CampaignForm {
    fn validate(&self) -> Result<(), validator::ValidationErrors> {
        if let Some(targeting) = &self.targeting {
            targeting.validate()?;
        };

        if self.start_date > self.end_date {
            return Err(validator::ValidationErrors::new());
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CampaignPatchForm {
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub targeting: TargetForm,
}
