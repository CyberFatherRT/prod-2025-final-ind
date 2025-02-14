use axum::Router;
use redis::Client as RedisClient;
use s3::{creds::Credentials, Bucket, BucketConfiguration, Region};
use solution::{app, AppState};
use sqlx::PgPool;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};
use tokio::sync::OnceCell;

type Container = testcontainers::ContainerAsync<GenericImage>;

static APP: OnceCell<Router> = OnceCell::const_new();

async fn setup_postgres() -> anyhow::Result<(PgPool, Container)> {
    let postgres = GenericImage::new("postgres", "17.2-alpine3.21")
        .with_exposed_port(5432.tcp())
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "password")
        .with_env_var("POSTGRES_DB", "postgres")
        .start()
        .await?;
    let port = postgres.get_host_port_ipv4(5432).await?;
    let database_url = format!("postgres://postgres:password@localhost:{port}/postgres");
    let pool = PgPool::connect(&database_url).await?;
    Ok((pool, postgres))
}

async fn setup_redis() -> anyhow::Result<(RedisClient, Container)> {
    let redis = GenericImage::new("redis", "7.2.7-alpine")
        .with_exposed_port(6379.tcp())
        .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
        .start()
        .await?;
    let port = redis.get_host_port_ipv4(6379).await?;
    let redis_url = format!("redis://localhost:{}", port);
    let rclient = RedisClient::open(redis_url)?;
    Ok((rclient, redis))
}

async fn setup_s3() -> anyhow::Result<(Bucket, Container)> {
    let s3 = GenericImage::new("bitnami/minio", "2025.2.7")
        .with_exposed_port(9000.tcp())
        .with_wait_for(WaitFor::message_on_stderr("API:"))
        .with_env_var("MINIO_ROOT_USER", "admin")
        .with_env_var("MINIO_ROOT_PASSWORD", "password")
        .start()
        .await?;

    let port = s3.get_host_port_ipv4(9000).await?;
    let credentials = Credentials::new(Some("admin"), Some("password"), None, None, None).unwrap();
    let bucket_name = "test-bucket";
    let region = Region::Custom {
        region: "us-east-1".to_string(),
        endpoint: format!("http://localhost:{}", port),
    };

    let mut bucket =
        Bucket::new(bucket_name, region.clone(), credentials.clone())?.with_path_style();

    if !bucket.exists().await? {
        bucket = Bucket::create_with_path_style(
            bucket_name,
            region,
            credentials,
            BucketConfiguration::default(),
        )
        .await?
        .bucket;
    }

    Ok((*bucket, s3))
}

pub async fn get_app() -> Router {
    APP.get_or_init(|| async {
        let ((pg, _), (redis, _), (s3, _)) =
            tokio::try_join!(setup_postgres(), setup_redis(), setup_s3()).unwrap();

        let state = AppState {
            pool: pg.clone(),
            rclient: redis.clone(),
            s3,
        };

        app(state)
    })
    .await
    .clone()
}
