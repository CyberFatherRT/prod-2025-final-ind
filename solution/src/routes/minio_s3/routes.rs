use std::path;

use axum::{
    body::Body,
    extract::{Multipart, Path, State},
    response::Response,
};
use reqwest::header::CONTENT_TYPE;
use tokio::try_join;
use tracing::info;
use uuid::Uuid;

use crate::{
    controllers::s3::UploadFileController,
    db::Db,
    errors::ProdError,
    models::{
        campaigns::{CampaignController, CampaignModel},
        s3::FilesModel,
    },
    AppState,
};

/// Add multiple files to a campaign
#[utoipa::path(
    post,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/file"
)]
pub async fn upload_file(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id)): Path<(Uuid, Uuid)>,
    mut multipart: Multipart,
) -> Result<(), ProdError> {
    let mut conn = state.pool.conn().await?;
    let mut files = vec![];

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| validator::ValidationErrors::new())?
    {
        let name = field
            .file_name()
            .map_or_else(|| "upload.bin".to_string(), ToOwned::to_owned);

        let content_type = field
            .content_type()
            .map_or_else(|| "application/octet-stream".to_string(), ToOwned::to_owned);

        let file_name = secure_path(&format!("{advertiser_id}/{campaign_id}"), &name);

        let bytes = field
            .bytes()
            .await
            .map_err(|_| validator::ValidationErrors::new())?;

        info!("Content type: {:?}", content_type);
        FilesModel::upload_file(&state, &file_name, content_type, bytes).await?;

        files.push(name);
    }

    CampaignModel::add_files(&mut conn, advertiser_id, campaign_id, files).await?;

    Ok(())
}

/// Dowload a file from campaign
#[utoipa::path(
    get,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/file/{file_name}"
)]
pub async fn download_file(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id, file_name)): Path<(Uuid, Uuid, String)>,
) -> Result<Response, ProdError> {
    let file_name = secure_path(&format!("{advertiser_id}/{campaign_id}"), &file_name);

    let (stream, content_type) = FilesModel::get_file(&state, &file_name).await?;
    let response = Response::builder()
        .header(CONTENT_TYPE, content_type)
        .body(Body::from_stream(stream))
        .map_err(|e| ProdError::Unknown(e.into()))?;

    Ok(response)
}

/// Deletes a file from the campaign
#[utoipa::path(
    delete,
    tag = "Campaigns",
    path = "/advertisers/{advertiser_id}/campaigns/{campaign_id}/file/{file_name}"
)]
pub async fn delete_file(
    State(state): State<AppState>,
    Path((advertiser_id, campaign_id, file_name)): Path<(Uuid, Uuid, String)>,
) -> Result<(), ProdError> {
    let mut conn = state.pool.conn().await?;
    let name = secure_path(&format!("{advertiser_id}/{campaign_id}"), &file_name);

    let s3_remove = FilesModel::delete_file(&state, &name);
    let db_remove = CampaignModel::delete_file(&mut conn, advertiser_id, campaign_id, &file_name);

    try_join!(s3_remove, db_remove)?;

    Ok(())
}

fn secure_path(base_path: &str, file_name: &str) -> String {
    let safe_file_name = path::Path::new(file_name).file_name().map_or_else(
        || "upload.bin".to_string(),
        |name| name.to_string_lossy().into_owned(),
    );

    format!("{}/{}", base_path.trim_end_matches('/'), safe_file_name)
}
