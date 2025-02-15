pub mod routes;

use axum::{routing::get, Router};
use routes::{
    get_advertiser_daily_statistics, get_advertiser_statistics, get_campaign_daily_statistics,
    get_campaign_statistics,
};

use crate::AppState;

pub fn get_routes() -> Router<AppState> {
    Router::new()
        .route("/campaign/{campaign_id}", get(get_campaign_statistics))
        .route(
            "/advertiser/{advertiser_id}/campaigns",
            get(get_advertiser_statistics),
        )
        .route(
            "/campaign/{campaign_id}/daily",
            get(get_campaign_daily_statistics),
        )
        .route(
            "/advertiser/{advertiser_id}/campaigns/daily",
            get(get_advertiser_daily_statistics),
        )
}
