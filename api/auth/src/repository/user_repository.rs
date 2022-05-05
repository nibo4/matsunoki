use crate::model::user::{User, UserId};
use crate::repository::meta::Repository;
#[cfg(test)]
use crate::repository::meta::ResolveError;

use async_trait::async_trait;
#[cfg(test)]
use mockall::mock;
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

pub trait HaveUserRepository {
    type UserRepository: UserRepository + Send + Sync + 'static;
    fn user_repository(&self) -> Self::UserRepository;
}

#[cfg(test)]
mock! {
    pub UserRepository {}

    #[async_trait]
    impl Repository<UserId, User> for UserRepository {
        async fn resolve(&self, id: &UserId) -> Result<Option<User>, ResolveError>;
    }

    #[async_trait]
    impl UserRepository for UserRepository {
        async fn store(&self, u: User) -> Result<(), StoreError>;
    }
}
