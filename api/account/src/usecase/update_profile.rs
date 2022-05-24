use crate::ability::profile_creator::ProfileCreator;
use crate::actor::user::User;
use crate::model::profile::entity::ProfileInvalidity;
use crate::model::user_profile::{UserProfile, UserProfileId};
#[cfg(test)]
use crate::repository::user_profile_repository::MockUserProfileRepository;
use crate::repository::user_profile_repository::StoreError;
use crate::repository::user_profile_repository::{
    HaveUserProfileRepository, UserProfileRepository,
};

use async_trait::async_trait;
use derive_more::Constructor;
use semval::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
use mockall::mock;

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct UpdateProfileUseCaseResult {
    pub user_profile: UserProfile,
}

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct UpdateProfileUseCaseParams {
    user_name: String,
    display_name: String,
    avatar_url: String,
}

#[derive(Debug, Error)]
pub enum UpdateProfileUseCaseError {
    #[error(transparent)]
    StoreError(#[from] StoreError),
    #[error("validation error: {0:?}")]
    ProfileValidationError(ValidationContext<ProfileInvalidity>),
}

#[async_trait]
pub trait UpdateProfileUseCase: HaveUserProfileRepository {
    async fn execute(
        &self,
        actor: &User,
        params: UpdateProfileUseCaseParams,
    ) -> Result<UpdateProfileUseCaseResult, UpdateProfileUseCaseError> {
        let profile = actor
            .create_profile(params.user_name, params.display_name, params.avatar_url)
            .map_err(|e| UpdateProfileUseCaseError::ProfileValidationError(e))?;
        let user_profile = UserProfile::new(UserProfileId::from(actor.0.clone()), profile);
        self.user_profile_repository().store(&user_profile).await?;
        Ok(UpdateProfileUseCaseResult::new(user_profile))
    }
}

impl<T: HaveUserProfileRepository> UpdateProfileUseCase for T {}

#[cfg(test)]
mock! {
    pub UpdateProfileUseCase {}

    impl HaveUserProfileRepository for UpdateProfileUseCase {
        type UserProfileRepository = MockUserProfileRepository;
        fn user_profile_repository(&self) -> &MockUserProfileRepository;
    }
}

#[cfg(test)]
mod tests {
    use super::{
        UpdateProfileUseCase, UpdateProfileUseCaseError, UpdateProfileUseCaseParams,
        UpdateProfileUseCaseResult,
    };
    use crate::actor::user::*;
    use crate::model::user_profile::UserProfileId;
    use crate::repository::user_profile_repository::{
        HaveUserProfileRepository, MockUserProfileRepository, StoreError,
    };
    use derive_more::Constructor;

    #[derive(Constructor)]
    struct UC {
        user_profile_repository: MockUserProfileRepository,
    }

    impl HaveUserProfileRepository for UC {
        type UserProfileRepository = MockUserProfileRepository;
        fn user_profile_repository(&self) -> &Self::UserProfileRepository {
            &self.user_profile_repository
        }
    }

    #[tokio::test]
    async fn update_profile_usecase_return_to_profile() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository.expect_store().returning(|_| Ok(()));

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase
            .execute(
                &user,
                UpdateProfileUseCaseParams::new(
                    "xxxx".to_string(),
                    "XXXX".to_string(),
                    "https://example.com".to_string(),
                ),
            )
            .await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().user_profile.id,
            UserProfileId::new(user.0 .0)
        )
    }

    #[tokio::test]
    async fn update_profile_usecase_is_err_when_store_error() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository
            .expect_store()
            .returning(|_| Err(StoreError::Unexpected(anyhow::anyhow!("a"))));

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase
            .execute(
                &user,
                UpdateProfileUseCaseParams::new(
                    "xxxx".to_string(),
                    "XXXX".to_string(),
                    "https://example.com".to_string(),
                ),
            )
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn update_profile_usecase_is_err_when_validation_error() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository.expect_store().returning(|_| Ok(()));

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase
            .execute(
                &user,
                UpdateProfileUseCaseParams::new(
                    "x".to_string(),
                    "XXXX".to_string(),
                    "https://example.com".to_string(),
                ),
            )
            .await;
        assert!(result.is_err());
    }
}
