use axum::Router;
use minio::s3::args::BucketExistsArgs;
use minio::s3::args::MakeBucketArgs;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::ClientBuilder;
use redis::Client as RedisClient;
use solution::{app, AppState};
use sqlx::PgPool;
use testcontainers::{
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
    GenericImage, ImageExt,
};
use tracing::info;

type Container = testcontainers::ContainerAsync<GenericImage>;

#[allow(dead_code)]
pub struct GlobalContainers {
    postgres_container: Container,
    pub postgres_port: u16,
    redis_container: Container,
    pub redis_port: u16,
    s3_container: Container,
    pub s3_port: u16,
}

pub async fn initialize_containers() -> GlobalContainers {
    let postgres_future = async {
        info!("Starting postgres container");
        let container = GenericImage::new("postgres", "17.2-alpine3.21")
            .with_exposed_port(5432.tcp())
            .with_wait_for(WaitFor::message_on_stderr(
                "database system is ready to accept connections",
            ))
            .with_env_var("POSTGRES_USER", "postgres")
            .with_env_var("POSTGRES_PASSWORD", "password")
            .with_env_var("POSTGRES_DB", "postgres")
            .start()
            .await
            .unwrap();
        let port = container.get_host_port_ipv4(5432).await.unwrap();
        info!("Postgres container started on port {}", port);
        Ok((container, port)) as anyhow::Result<_>
    };

    let redis_future = async {
        info!("Starting redis container");
        let container = GenericImage::new("redis", "7.2.7-alpine")
            .with_exposed_port(6379.tcp())
            .with_wait_for(WaitFor::message_on_stdout("Ready to accept connections"))
            .start()
            .await
            .unwrap();
        let port = container.get_host_port_ipv4(6379).await.unwrap();
        info!("Redis container started on port {}", port);
        Ok((container, port)) as anyhow::Result<_>
    };

    let s3_future = async {
        info!("Starting s3 (minio) container");
        let container = GenericImage::new("bitnami/minio", "2025.2.7")
            .with_exposed_port(9000.tcp())
            .with_wait_for(WaitFor::message_on_stderr("API:"))
            .with_env_var("MINIO_ROOT_USER", "admin")
            .with_env_var("MINIO_ROOT_PASSWORD", "password")
            .start()
            .await
            .unwrap();
        let port = container.get_host_port_ipv4(9000).await.unwrap();
        info!("S3 (minio) container started on port {}", port);
        Ok((container, port)) as anyhow::Result<_>
    };

    let (
        (postgres_container, postgres_port),
        (redis_container, redis_port),
        (s3_container, s3_port),
    ) = tokio::try_join!(postgres_future, redis_future, s3_future).unwrap();

    GlobalContainers {
        postgres_container,
        postgres_port,
        redis_container,
        redis_port,
        s3_container,
        s3_port,
    }
}

pub async fn get_app(global: &GlobalContainers) -> Router {
    let postgres_port = global.postgres_port;
    let redis_port = global.redis_port;
    let s3_port = global.s3_port;

    let database_url = format!(
        "postgres://postgres:password@localhost:{}/postgres",
        postgres_port
    );

    info!("Connecting to Postgres at {}", database_url);
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let redis_url = format!("redis://localhost:{}", redis_port);
    info!("Connecting to Redis at {}", redis_url);
    let rclient = RedisClient::open(redis_url).expect("Failed to create Redis client");

    let s3_endpoint = format!("http://localhost:{}", s3_port);
    info!("Connecting to S3 (minio) at {}", s3_endpoint);
    let bucket_name = "test";

    let static_provider = StaticProvider::new("admin", "password", None);

    let client = ClientBuilder::new(s3_endpoint.parse::<BaseUrl>().unwrap())
        .provider(Some(Box::new(static_provider)))
        .build()
        .unwrap();

    let exists = client
        .bucket_exists(&BucketExistsArgs::new(bucket_name).unwrap())
        .await
        .unwrap();

    if !exists {
        client
            .make_bucket(&MakeBucketArgs::new(bucket_name).unwrap())
            .await
            .unwrap();
    };

    let state = AppState {
        pool,
        rclient,
        s3: client,
        bucket_name: bucket_name.to_string(),
    };

    app(state)
}

pub fn init_tracing() {
    tracing_subscriber::fmt().with_target(true).init()
}
