use async_trait::async_trait;
use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::errors::ProdError;

#[async_trait]
pub trait Db {
    type Conn;

    async fn conn(&self) -> Result<Self::Conn, ProdError>;
}

#[async_trait]
impl Db for PgPool {
    type Conn = PoolConnection<Postgres>;

    async fn conn(&self) -> Result<Self::Conn, ProdError> {
        self.acquire().await.map_err(ProdError::DatabaseError)
    }
}
