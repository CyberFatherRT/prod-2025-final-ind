use minio::s3::{
    args::{BucketExistsArgs, MakeBucketArgs},
    creds::StaticProvider,
    http::BaseUrl,
    Client, ClientBuilder,
};

use crate::utils::env;

pub async fn setup_s3() -> anyhow::Result<(Client, String)> {
    let bucket_name = env("MINIO_BUCKET_NAME");
    let endpoint = env("MINIO_ENDPOINT");

    let static_provider = StaticProvider::new(
        &env("MINIO_ACCESS_KEY_ID"),
        &env("MINIO_SECRET_ACCESS_KEY"),
        None,
    );

    let client = ClientBuilder::new(endpoint.parse::<BaseUrl>()?)
        .provider(Some(Box::new(static_provider)))
        .build()?;

    let exists = client
        .bucket_exists(&BucketExistsArgs::new(&bucket_name)?)
        .await?;

    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(&bucket_name)?)
            .await?;
    }

    Ok((client, bucket_name))
}
