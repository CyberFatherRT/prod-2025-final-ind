use sqlx::{pool::PoolConnection, PgPool, Postgres};

use crate::errors::ProdError;

pub trait Db {
    type Conn;
    async fn conn(&self) -> Result<Self::Conn, ProdError>;
}

impl Db for PgPool {
    type Conn = PoolConnection<Postgres>;

    async fn conn(&self) -> Result<Self::Conn, ProdError> {
        self.acquire().await.map_err(ProdError::DatabaseError)
    }
}
