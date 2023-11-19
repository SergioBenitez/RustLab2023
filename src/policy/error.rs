use thiserror::Error;

#[derive(Error, Debug)]
pub enum PolicyError {
    #[error("db error: {0}")]
    Db(#[from] sqlx::Error),
    #[error("password error: {0}")]
    Password(#[from] password_hash::Error),
    #[error("account is inactive")]
    Inactive,
    #[error("data in database is semantically invalid")]
    InvalidData,
    #[error("authorization check failed")]
    Unauthorized,
}
