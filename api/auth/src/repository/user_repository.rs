use crate::model::login_provider::IdInProvider;
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
    #[error("{kind} is unsupported provider kind")]
    UnSupportedProviderKind { kind: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum FilterByIdInProviderError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserRepository: Repository<UserId, User> {
    async fn store(&self, u: &User) -> Result<(), StoreError>;
    async fn find_by_id_in_provider(
        &self,
        id_in_provider: &IdInProvider,
    ) -> Result<Option<User>, FilterByIdInProviderError>;
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
        async fn find_by_id_in_provider(&self, id_in_provider: &IdInProvider) -> Result<Option<User>, FilterByIdInProviderError>;
        async fn store(&self, u: &User) -> Result<(), StoreError>;
    }
}
