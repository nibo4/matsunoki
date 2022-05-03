use std::result::Result as StdResult;
use thiserror::Error;

pub type Result<T> = StdResult<T, AuthError>;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error(transparent)]
    Other(#[from] anyhow::Error),  // source and Display delegate to anyhow::Error
}

