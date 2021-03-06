use crate::model::user_profile::{UserProfile, UserProfileId};
use crate::repository::meta::Repository;
#[cfg(test)]
use crate::repository::meta::ResolveError;
use async_trait::async_trait;
use thiserror::Error;

#[cfg(test)]
use mockall::mock;

#[derive(Error, Debug)]
pub enum StoreError {
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait UserProfileRepository: Repository<UserProfileId, UserProfile> {
    async fn store(&self, profile: &UserProfile) -> Result<(), StoreError>;
}

pub trait HaveUserProfileRepository {
    type UserProfileRepository: UserProfileRepository + Send + Sync + 'static;
    fn user_profile_repository(&self) -> &Self::UserProfileRepository;
}

#[cfg(test)]
mock! {
    pub UserProfileRepository {}

    #[async_trait]
    impl Repository<UserProfileId, UserProfile> for UserProfileRepository {
        async fn resolve(&self, id: &UserProfileId) -> Result<Option<UserProfile>, ResolveError>;
    }

    #[async_trait]
    impl UserProfileRepository for UserProfileRepository {
        async fn store(&self, u: &UserProfile) -> Result<(), StoreError>;
    }
}
