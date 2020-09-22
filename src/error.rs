use thiserror::Error;

/// Enum listing possible authentication error codes.
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum AppError {
    /// If the request was invalid or malformed.
    #[error("the request was invalid {0}")]
    InvalidRequest(String),

    /// An error occured when connecting to or using the database.
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),

    /// Any other, unknown error sources.
    #[error("{0}")]
    Unknown(#[source] Box<dyn std::error::Error + Sync + Send>),
}
