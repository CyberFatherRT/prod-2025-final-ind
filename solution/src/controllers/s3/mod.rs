use std::{io::Error, pin::Pin};

use bytes::Bytes;
use futures_core::Stream;
use minio::s3::{args::PutObjectApiArgs, error, types::S3Api, utils::Multimap};
use tracing::info;

use crate::{errors::ProdError, models::s3::FilesModel, AppState};

pub trait UploadFileController {
    async fn upload_file(
        state: &AppState,
        name: &str,
        content_type: String,
        content: Bytes,
    ) -> Result<(), ProdError>;

    async fn get_file(
        state: &AppState,
        name: &str,
    ) -> Result<
        (
            Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>,
            String,
        ),
        ProdError,
    >;

    async fn delete_file(state: &AppState, name: &str) -> Result<(), ProdError>;
}

impl UploadFileController for FilesModel {
    async fn upload_file(
        state: &AppState,
        name: &str,
        content_type: String,
        content: Bytes,
    ) -> Result<(), ProdError> {
        let client = state.s3.clone();
        let bucket_name = state.bucket_name.clone();

        let mut args = PutObjectApiArgs::new(&bucket_name, name, content.as_ref())
            .expect("Failed to create PutObjectApiArgs");

        let headers = Multimap::from_iter(vec![("Content-Type".to_string(), content_type.clone())]);
        args.headers = Some(&headers);

        let response = client
            .put_object_api(&args)
            .await
            .map_err(|e| ProdError::Unknown(e.into()))?;

        info!("Response: {:?}", response);

        Ok(())
    }

    async fn get_file(
        state: &AppState,
        name: &str,
    ) -> Result<
        (
            Pin<Box<dyn Stream<Item = Result<Bytes, Error>> + Send>>,
            String,
        ),
        ProdError,
    > {
        let client = state.s3.clone();
        let bucket_name = state.bucket_name.clone();

        let response =
            client
                .get_object(&bucket_name, name)
                .send()
                .await
                .map_err(|err| match err {
                    error::Error::S3Error(err) if err.code == "NoSuchKey" => {
                        ProdError::NotFound(format!("File not found: {name}"))
                    }
                    _ => ProdError::Unknown(err.into()),
                })?;

        let headers = response.headers;
        let content_type = headers
            .get("Content-Type")
            .map_or("application/octet-stream", |v| {
                v.to_str().unwrap_or("application/octet-stream")
            });

        let (content, _) = response
            .content
            .to_stream()
            .await
            .map_err(|e| ProdError::Unknown(e.into()))?;

        Ok((content, content_type.to_string()))
    }

    async fn delete_file(state: &AppState, name: &str) -> Result<(), ProdError> {
        let client = state.s3.clone();
        let bucket_name = state.bucket_name.clone();

        let response = client
            .remove_object(&bucket_name, name)
            .send()
            .await
            .map_err(|e| ProdError::Unknown(e.into()))?;

        info!("Response: {:?}", response);

        Ok(())
    }
}
