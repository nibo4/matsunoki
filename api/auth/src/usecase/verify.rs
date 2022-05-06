use crate::driver::firebase_auth::{
    AccessToken, FirebaseAuthDriver, HaveFirebaseAuthDriver, VerifyError,
};
use crate::model::login_provider::IdInProvider;
use crate::model::user::User;
use crate::repository::user_repository::{
    FilterByIdInProviderError, HaveUserRepository, UserRepository,
};
use async_trait::async_trait;
use derive_more::Constructor;
use thiserror::Error;

#[cfg(test)]
use crate::driver::firebase_auth::MockFirebaseAuthDriver;
#[cfg(test)]
use crate::repository::user_repository::MockUserRepository;

#[derive(Debug, Constructor)]
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
trait VerifyUseCase: HaveUserRepository + HaveFirebaseAuthDriver {
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

#[cfg(test)]
mockall::mock! {
    pub VerifyUseCase {}

    impl VerifyUseCase for VerifyUseCase {}
    impl HaveUserRepository for VerifyUseCase {
        type UserRepository = MockUserRepository;
        fn user_repository(&self) -> MockUserRepository;
    }

    impl HaveFirebaseAuthDriver for VerifyUseCase {
        type FirebaseAuthDriver = MockFirebaseAuthDriver;
        fn firebase_auth(&self) -> MockFirebaseAuthDriver;
    }
}

#[cfg(test)]
mod tests {
    use super::VerifyUseCase;

    use crate::driver::firebase_auth::{
        HaveFirebaseAuthDriver, MockFirebaseAuthDriver, VerifyError, VerifyResult,
    };
    use crate::model::user::User;
    use crate::repository::user_repository::{HaveUserRepository, MockUserRepository};

    #[tokio::test]
    async fn verify_use_case_return_to_user_when_ok() {
        struct UC;
        impl VerifyUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_find_by_id_in_provider()
                    .returning(|_| Ok(Some(User::default())));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Ok(VerifyResult::default()));
                mock
            }
        }

        assert!(UC.execute("xxx").await.is_ok());
    }

    #[tokio::test]
    async fn verify_use_case_return_to_err_when_user_not_found() {
        struct UC;
        impl VerifyUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Ok(VerifyResult::default()));
                mock
            }
        }

        assert!(UC.execute("xxx").await.is_err());
        assert_eq!(
            UC.execute("xxx").await.unwrap_err().to_string(),
            "User is not found. ".to_string()
        )
    }

    #[tokio::test]
    async fn verify_use_case_return_to_err_when_verify_error() {
        struct UC;
        impl VerifyUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_find_by_id_in_provider()
                    .returning(|_| Ok(Some(User::default())));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Err(VerifyError::TokenExpired));
                mock
            }
        }

        assert!(UC.execute("xxx").await.is_err());
        assert_eq!(
            UC.execute("xxx").await.unwrap_err().to_string(),
            "Token expired.".to_string()
        )
    }
}
