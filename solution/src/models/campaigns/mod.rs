use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    PgConnection,
};
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    errors::ProdError,
    forms::campaigns::{CampaignForm, CampaignPatchForm, CampaignQuery},
};

#[derive(Debug, Serialize, Deserialize, Type, Clone, ToSchema)]
#[sqlx(type_name = "CAMPAIGN_GENDER", rename_all = "UPPERCASE")]
#[serde(rename_all = "UPPERCASE")]
pub enum CampaignGenderModel {
    Male,
    Female,
    Any,
}

#[derive(Debug, Serialize, Deserialize, FromRow, ToSchema)]
pub struct CampaignTargetModel {
    pub gender: Option<CampaignGenderModel>,
    pub age_from: Option<i32>,
    pub age_to: Option<i32>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CampaignModel {
    #[serde(rename = "campaign_id")]
    pub id: Uuid,
    pub advertiser_id: Uuid,
    pub impressions_limit: i32,
    pub clicks_limit: i32,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i32,
    pub end_date: i32,
    pub targeting: CampaignTargetModel,
}

#[derive(Clone, Debug, FromRow)]
pub struct CampaignRow {
    pub id: Uuid,
    pub advertiser_id: Uuid,
    pub impressions_limit: i32,
    pub clicks_limit: i32,
    pub cost_per_impression: f64,
    pub cost_per_click: f64,
    pub ad_title: String,
    pub ad_text: String,
    pub start_date: i32,
    pub end_date: i32,
    pub gender: Option<CampaignGenderModel>,
    pub age_from: Option<i32>,
    pub age_to: Option<i32>,
    pub location: Option<String>,
}

impl From<CampaignRow> for CampaignModel {
    fn from(row: CampaignRow) -> Self {
        Self {
            id: row.id,
            advertiser_id: row.advertiser_id,
            impressions_limit: row.impressions_limit,
            clicks_limit: row.clicks_limit,
            cost_per_impression: row.cost_per_impression,
            cost_per_click: row.cost_per_click,
            ad_title: row.ad_title,
            ad_text: row.ad_text,
            start_date: row.start_date,
            end_date: row.end_date,
            targeting: CampaignTargetModel {
                gender: row.gender,
                age_from: row.age_from,
                age_to: row.age_to,
                location: row.location,
            },
        }
    }
}

pub trait CampaignController {
    async fn create(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign: CampaignForm,
    ) -> Result<CampaignModel, ProdError>;

    async fn list(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        query: CampaignQuery,
    ) -> Result<Vec<CampaignModel>, ProdError>;

    async fn update(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
        campaign: CampaignPatchForm,
    ) -> Result<CampaignModel, ProdError>;

    async fn get(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
    ) -> Result<CampaignModel, ProdError>;

    async fn delete(
        conn: &mut PgConnection,
        advertiser_id: Uuid,
        campaign_id: Uuid,
    ) -> Result<(), ProdError>;
}
