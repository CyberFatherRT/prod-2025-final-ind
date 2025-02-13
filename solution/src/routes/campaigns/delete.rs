use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use uuid::Uuid;

use crate::{
    controllers::campaigns::CampaignController, db::Db, errors::ProdError,
    models::campaigns::CampaignModel, AppState,
};
