use crate::adapter::firebase_auth::{
    AccessToken, FirebaseAuthDriver, HaveFirebaseAuthDriver, VerifyError,
};
use crate::model::login_provider::IdInProvider;
use crate::model::user::User;
use crate::repository::user_repository::{
    FilterByIdInProviderError, HaveUserRepository, UserRepository,
};
use async_trait::async_trait;
use derive_more::Constructor;
use serde::Serialize;
use thiserror::Error;

#[cfg(test)]
use crate::adapter::firebase_auth::MockFirebaseAuthDriver;
#[cfg(test)]
use crate::repository::user_repository::MockUserRepository;

#[derive(Debug, Constructor, Serialize)]
pub struct VerifyUseCaseResult {
    pub user: User,
}

#[derive(Error, Debug)]
pub enum VerifyUseCaseError {
    #[error(transparent)]
    VerifyFailed(#[from] VerifyError),
    #[error(transparent)]
    FilterError(#[from] FilterByIdInProviderError),
    #[error("User is not found. {0}")]
    UserNotFound(String),
}

#[async_trait]
pub trait VerifyUseCase: HaveUserRepository + HaveFirebaseAuthDriver {
    async fn execute(&self, token: &str) -> Result<VerifyUseCaseResult, VerifyUseCaseError> {
        let verify_result = self
            .firebase_auth()
            .verify(AccessToken::new(token.to_string()))
            .await?;
        let provider_id = IdInProvider::new(verify_result.uid.0);
        let user = match self
            .user_repository()
            .find_by_id_in_provider(&provider_id)
            .await?
        {
            Some(u) => u,
            None => return Err(VerifyUseCaseError::UserNotFound(provider_id.0)),
        };
        Ok(VerifyUseCaseResult::new(user))
    }
}

impl<T: HaveUserRepository + HaveFirebaseAuthDriver> VerifyUseCase for T {}

#[cfg(test)]
mockall::mock! {
    pub VerifyUseCase {}

    impl HaveUserRepository for VerifyUseCase {
        type UserRepository = MockUserRepository;
        fn user_repository(&self) -> &MockUserRepository;
    }

    impl HaveFirebaseAuthDriver for VerifyUseCase {
        type FirebaseAuthDriver = MockFirebaseAuthDriver;
        fn firebase_auth(&self) -> &MockFirebaseAuthDriver;
    }
}

#[cfg(test)]
mod tests {
    use super::VerifyUseCase;

    use crate::adapter::firebase_auth::{
        HaveFirebaseAuthDriver, MockFirebaseAuthDriver, VerifyError, VerifyResult,
    };
    use crate::model::user::User;
    use crate::repository::user_repository::{HaveUserRepository, MockUserRepository};
    use derive_more::Constructor;

    #[derive(Constructor)]
    struct UC {
        user_repo: MockUserRepository,
        firebase_auth: MockFirebaseAuthDriver,
    }
    impl HaveUserRepository for UC {
        type UserRepository = MockUserRepository;
        fn user_repository(&self) -> &Self::UserRepository {
            &self.user_repo
        }
    }

    impl HaveFirebaseAuthDriver for UC {
        type FirebaseAuthDriver = MockFirebaseAuthDriver;
        fn firebase_auth(&self) -> &Self::FirebaseAuthDriver {
            &self.firebase_auth
        }
    }

    #[tokio::test]
    async fn verify_use_case_return_to_user_when_ok() {
        let mut user_repository = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();

        user_repository
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(Some(User::default())));
        firebase_auth
            .expect_verify()
            .returning(|_| Ok(VerifyResult::default()));

        assert!(UC::new(user_repository, firebase_auth)
            .execute("xxx")
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn verify_use_case_return_to_err_when_user_not_found() {
        let mut user_repository = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();

        user_repository
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        firebase_auth
            .expect_verify()
            .returning(|_| Ok(VerifyResult::default()));

        let usecase_result = UC::new(user_repository, firebase_auth).execute("xxx").await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.unwrap_err().to_string(),
            "User is not found. ".to_string()
        )
    }

    #[tokio::test]
    async fn verify_use_case_return_to_err_when_verify_error() {
        let mut user_repository = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();

        user_repository
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(Some(User::default())));
        firebase_auth
            .expect_verify()
            .returning(|_| Err(VerifyError::TokenExpired));

        let usecase_result = UC::new(user_repository, firebase_auth).execute("xxx").await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.unwrap_err().to_string(),
            "Token expired.".to_string()
        )
    }
}
