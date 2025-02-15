use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(thiserror::Error, Debug)]
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

    /// Conflict Error
    #[error("conflict")]
    Conflict(String),

    /// Forbidden Error
    #[error("{0}")]
    Forbidden(String),

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error + Send + Sync>),
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
            ProdError::Forbidden(_) => builder.status(StatusCode::FORBIDDEN),
            ProdError::Conflict(_) => builder.status(StatusCode::CONFLICT),
            ProdError::NotFound(_) => builder.status(StatusCode::NOT_FOUND),
        }
        .body(error)
        .expect("Failed to build response")
    }
}

impl IntoResponse for ProdError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            Self::AlreadyExists(_) | Self::InvalidRequest(_) => {
                (StatusCode::BAD_REQUEST, self.to_string())
            }
            Self::DatabaseError(_) | Self::RedisError(_) | Self::Unknown(_) => {
                (StatusCode::INTERNAL_SERVER_ERROR, self.to_string())
            }
            Self::Forbidden(_) => (StatusCode::FORBIDDEN, self.to_string()),
            Self::Conflict(_) => (StatusCode::CONFLICT, self.to_string()),
            Self::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
        };

        (status, Json(serde_json::json!({ "error": message }))).into_response()
    }
}
