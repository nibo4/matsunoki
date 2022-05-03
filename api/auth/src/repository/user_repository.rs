use crate::model::user::{User, UserId};
use crate::repository::internal::Repository;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("id: {id} entity is not exist")]
    AlreadyExist { id: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Repository<UserId, User> {
    async fn store(&self, u: User) -> Result<(), StoreError>;
}
