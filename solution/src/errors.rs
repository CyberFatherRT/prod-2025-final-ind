use axum::{http::StatusCode, response::Response};

#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum ProdError {
    /// If the request was invalid or malformed.
    #[error("the request was invalid {0}")]
    InvalidRequest(#[from] validator::ValidationErrors),

    /// If the username and password combination did not match when attempting to authenticate.
    #[error("invalid username or password")]
    InvalidUsernameOrPassword,

    /// If a registration was attemted, but the email address already exists in the database.
    #[error("a user with the email {0} already exists")]
    UserAlreadyExists(String),

    /// An error occured when validating or generating a JWT.
    #[error("invalid token")]
    InvalidToken(#[from] jsonwebtoken::errors::Error),

    /// An error occured when connection to or using the database.
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),

    /// An error occured with the Argon2id hashing implementation.
    #[error("hashing error")]
    HashingError(#[from] argon2::Error),

    /// Not found error
    #[error("not found")]
    NotFound(String),

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error>),
}

impl From<ProdError> for Response<String> {
    fn from(prod_error: ProdError) -> Self {
        let error = format!("{:?}", prod_error);
        let builder = Response::builder();
        match prod_error {
            ProdError::InvalidRequest(_) => builder.status(StatusCode::BAD_REQUEST),
            ProdError::InvalidUsernameOrPassword => builder.status(StatusCode::UNAUTHORIZED),
            ProdError::UserAlreadyExists(_) => builder.status(StatusCode::CONFLICT),
            ProdError::InvalidToken(_) => builder.status(StatusCode::UNAUTHORIZED),
            ProdError::DatabaseError(_) => builder.status(StatusCode::INTERNAL_SERVER_ERROR),
            ProdError::HashingError(_) => builder.status(StatusCode::INTERNAL_SERVER_ERROR),
            ProdError::NotFound(_) => builder.status(StatusCode::NOT_FOUND),
            ProdError::Unknown(_) => builder.status(StatusCode::INTERNAL_SERVER_ERROR),
        }
        .body(error)
        .unwrap()
    }
}
