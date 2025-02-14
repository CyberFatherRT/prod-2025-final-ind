use axum::{http::StatusCode, response::Response};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ProdError {
    /// If the request was invalid or malformed.
    #[error("the request was invalid {0}")]
    InvalidRequest(#[from] validator::ValidationErrors),

    #[error("{0}")]
    AlreadyExists(String),

    /// An error occured when connection to or using the database.
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),

    /// An error occured when connection to or using the redis.
    #[error("redis error")]
    RedisError(#[from] redis::RedisError),

    /// Not found error
    #[error("not found")]
    NotFound(String),

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error>),
}

impl From<ProdError> for Response<String> {
    fn from(prod_error: ProdError) -> Self {
        let error = format!("{prod_error:?}");
        let builder = Response::builder();
        match prod_error {
            ProdError::AlreadyExists(_) | ProdError::InvalidRequest(_) => {
                builder.status(StatusCode::BAD_REQUEST)
            }
            ProdError::RedisError(_) | ProdError::Unknown(_) | ProdError::DatabaseError(_) => {
                builder.status(StatusCode::INTERNAL_SERVER_ERROR)
            }
            ProdError::NotFound(_) => builder.status(StatusCode::NOT_FOUND),
        }
        .body(error)
        .expect("Failed to build response")
    }
}
