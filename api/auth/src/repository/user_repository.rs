use crate::model::user::{User, UserId};
use crate::repository::meta::Repository;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error("id: {id} entity is not exist")]
    AlreadyExist { id: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait UserRepository: Repository<UserId, User> {
    async fn store(&self, u: User) -> Result<(), StoreError>;
}

pub trait HaveUserRepository {
    type UserRepository: UserRepository + Send + Sync + 'static;
    fn user_repository(&self) -> Self::UserRepository;
}
