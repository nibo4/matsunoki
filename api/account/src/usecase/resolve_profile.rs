use crate::actor::user::User;
use crate::model::user_profile::{UserProfile, UserProfileId};
use crate::repository::meta::{Repository, ResolveError};
use crate::repository::user_profile_repository::HaveUserProfileRepository;
#[cfg(test)]
use crate::repository::user_profile_repository::MockUserProfileRepository;

use async_trait::async_trait;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[cfg(test)]
use mockall::mock;

#[derive(Debug, Constructor, Serialize, Deserialize)]
pub struct ResolveProfileUseCaseResult {
    pub user_profile: UserProfile,
}

#[derive(Debug, Error)]
pub enum ResolveProfileUseCaseError {
    #[error(transparent)]
    ResolveError(#[from] ResolveError),
    #[error("user_profile is not found")]
    NotFound,
}

#[async_trait]
pub trait ResolveProfileUseCase: HaveUserProfileRepository {
    async fn execute(
        &self,
        actor: &User,
    ) -> Result<ResolveProfileUseCaseResult, ResolveProfileUseCaseError> {
        let id: UserProfileId = actor.0.clone().into();
        let profile = self.user_profile_repository().resolve(&id).await?;
        match profile {
            Some(p) => Ok(ResolveProfileUseCaseResult::new(p)),
            None => Err(ResolveProfileUseCaseError::NotFound),
        }
    }
}

impl<T: HaveUserProfileRepository> ResolveProfileUseCase for T {}

#[cfg(test)]
mock! {
    pub ResolveProfileUseCase {}

    impl HaveUserProfileRepository for ResolveProfileUseCase {
        type UserProfileRepository = MockUserProfileRepository;
        fn user_profile_repository(&self) -> &MockUserProfileRepository;
    }
}

#[cfg(test)]
mod tests {
    use crate::actor::user::*;
    use crate::model::profile::avatar::Avatar;
    use crate::model::profile::display_name::DisplayName;
    use crate::model::profile::entity::Profile;
    use crate::model::profile::user_name::UserName;
    use crate::model::user_profile::{UserProfile, UserProfileId};
    use crate::repository::meta::ResolveError;
    use crate::repository::user_profile_repository::{
        HaveUserProfileRepository, MockUserProfileRepository,
    };
    use crate::usecase::resolve_profile::ResolveProfileUseCase;
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
    async fn resolve_profile_return_to_profile() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository.expect_resolve().returning(|_| {
            Ok(Some(UserProfile::new(
                UserProfileId::new("fooo".to_string()),
                Profile::new(
                    UserName::new("foo".to_string()),
                    DisplayName::new("fooo".to_string()),
                    Avatar::new("avatar".to_string()),
                ),
            )))
        });

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase.execute(&user).await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap().user_profile.id,
            UserProfileId::new("fooo".to_string())
        )
    }

    #[tokio::test]
    async fn resolve_profile_return_to_none() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository
            .expect_resolve()
            .returning(|_| Ok(None));

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase.execute(&user).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn resolve_profile_return_to_error() {
        let mut user_profile_repository = MockUserProfileRepository::new();
        user_profile_repository
            .expect_resolve()
            .returning(|_| Err(ResolveError::Unexpected(anyhow::anyhow!("foo"))));

        let usecase = UC::new(user_profile_repository);
        let user = User::default();
        let result = usecase.execute(&user).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "foo");
    }
}
