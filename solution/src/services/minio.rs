use aws_sdk_s3::Client;

use crate::utils::env;

pub async fn setup_s3() -> Client {
    let bucket_name = env("BUCKET_NAME");
    let endpoint = env("MINIO_ENDPOINT");

    let config = aws_config::from_env()
        .endpoint_url(endpoint)
        .region("ru-central-1")
        .load()
        .await;
    let client = Client::new(&config);

    let _ = client.create_bucket().bucket(bucket_name).send().await;

    client
}
