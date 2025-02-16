use crate::routes::{
    advertisement::routes::{__path_click_ad, __path_get_ad},
    advertisers::routes::{
        __path_bulk as __path_advertiser_bulk, __path_get_advertiser_by_id, __path_ml_scores,
    },
    campaigns::routes::{
        __path_create, __path_delete_campaign, __path_get_campaign_by_id, __path_list,
        __path_update,
    },
    clients::routes::{__path_bulk, __path_get_client_by_id},
    minio_s3::routes::{__path_delete_file, __path_download_file, __path_upload_file},
    statistics::routes::{
        __path_get_advertiser_daily_statistics, __path_get_advertiser_statistics,
        __path_get_campaign_daily_statistics, __path_get_campaign_statistics,
    },
    time::routes::__path_set_date,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        get_client_by_id, bulk,
        upload_file, download_file, delete_file,
        create, list, get_campaign_by_id, update, delete_campaign,
        click_ad, get_ad,
        advertiser_bulk, get_advertiser_by_id, ml_scores,
        get_advertiser_statistics, get_campaign_statistics,
        get_campaign_daily_statistics, get_advertiser_daily_statistics,
        set_date,
    ),
    tags(
        (name = "Clients", description = "Client managing: creation and info updating"),
        (name = "Advertisers", description = "Advertiser managing: creation and info updating"),
        (name = "Campaigns", description = "Campaign managing: create, update, delete and list campaigns"),
        (name = "Ads", description = "Showing ads to clients and recording clicks"),
        (name = "Statistics", description = "Get statistics for campaigns and advertisers, daily and aggregated"),
        (name = "Time", description = "Manage time for the system"),
    )
)]
pub struct ApiDoc;
