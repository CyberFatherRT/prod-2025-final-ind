use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};

use crate::utils::env;

pub async fn setup_s3() -> anyhow::Result<Bucket> {
    let access_key_id = env("MINIO_ACCESS_KEY_ID");
    let secret_access_key = env("MINIO_SECRET_ACCESS_KEY");
    let bucket_name = env("MINIO_BUCKET_NAME");
    let endpoint = env("MINIO_ENDPOINT");

    let credentials = Credentials::new(
        Some(&access_key_id),
        Some(&secret_access_key),
        None,
        None,
        None,
    )?;

    let region = Region::Custom {
        region: "us-east-1".to_string(),
        endpoint,
    };

    let mut bucket =
        Bucket::new(&bucket_name, region.clone(), credentials.clone())?.with_path_style();

    if !bucket.exists().await? {
        bucket = Bucket::create_with_path_style(
            &bucket_name,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await?
        .bucket;
    }

    Ok(*bucket)
}
