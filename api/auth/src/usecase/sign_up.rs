#[cfg(test)]
use crate::driver::firebase_auth::MockFirebaseAuthDriver;
#[cfg(test)]
use crate::repository::user_repository::MockUserRepository;
use crate::{
    driver::firebase_auth::{AccessToken, FirebaseAuthDriver, HaveFirebaseAuthDriver, VerifyError},
    model::meta::Entity,
    model::user::{User, UserId},
    repository::meta::{Repository, ResolveError},
    repository::user_repository::{HaveUserRepository, StoreError, UserRepository},
};
use async_trait::async_trait;
use derive_more::Constructor;
use thiserror::Error;

#[derive(Debug, Constructor)]
pub struct SignUpUseCaseResult {}

#[derive(Error, Debug)]
pub enum SignUpUseCaseError {
    #[error(transparent)]
    VerifyFailed(#[from] VerifyError),
    #[error(transparent)]
    ResolveError(#[from] ResolveError),
    #[error(transparent)]
    StoreError(#[from] StoreError),
    #[error("User is already exist. (id: {0})")]
    AlreadyExist(String),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait SignUpUseCase: HaveUserRepository + HaveFirebaseAuthDriver {
    async fn execute(&self, token: String) -> Result<SignUpUseCaseResult, SignUpUseCaseError> {
        let verify_result = self.firebase_auth().verify(AccessToken::new(token)).await?;
        let user_id = UserId::new(verify_result.uid.0);

        if let Some(user) = self.user_repository().resolve(&user_id).await? {
            return Err(SignUpUseCaseError::AlreadyExist(user.id().0.clone()));
        }

        let sign_up_user = User::new(user_id);

        self.user_repository().store(sign_up_user).await?;
        Ok(SignUpUseCaseResult::new())
    }
}

#[cfg(test)]
mockall::mock! {
    pub SignUpUseCase {}

    impl HaveUserRepository for SignUpUseCase {
        type UserRepository = MockUserRepository;
        fn user_repository(&self) -> MockUserRepository;
    }

    impl HaveFirebaseAuthDriver for SignUpUseCase {
        type FirebaseAuthDriver = MockFirebaseAuthDriver;
        fn firebase_auth(&self) -> MockFirebaseAuthDriver;
    }
}

#[cfg(test)]
mod tests {
    use super::SignUpUseCase;
    use crate::driver::firebase_auth::{
        FirebaseAuthDriver, FullName, HaveFirebaseAuthDriver, LocalId, MockFirebaseAuthDriver,
        VerifyError, VerifyResult,
    };
    use crate::model::user::{User, UserId};
    use crate::repository::user_repository::{HaveUserRepository, MockUserRepository, StoreError};

    #[tokio::test]
    async fn sign_up_return_ok_when_verify_ok_and_user_repository_return_empty() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify().returning(|_| {
                    Ok(VerifyResult::new(
                        LocalId::new("DUMMY".to_string()),
                        FullName::new("FULL NAME".to_string()),
                    ))
                });
                mock
            }
        }

        assert!(UC().execute("xxxx".to_string()).await.is_ok())
    }

    #[tokio::test]
    async fn sign_up_return_err_when_failed_store() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| {
                    Err(StoreError::AlreadyExist {
                        id: "foo".to_string(),
                    })
                });
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify().returning(|_| {
                    Ok(VerifyResult::new(
                        LocalId::new("DUMMY".to_string()),
                        FullName::new("FULL NAME".to_string()),
                    ))
                });
                mock
            }
        }

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "id: foo entity is already exist".to_string()
        )
    }

    #[tokio::test]
    async fn sign_up_return_err_when_token_expire() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
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

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Token expired.".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_user_disabled() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Err(VerifyError::UserDisabled(LocalId::new("foo".to_string()))));
                mock
            }
        }

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Provider user disabled.(id: foo)".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_user_not_found() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Err(VerifyError::UserNotFound(LocalId::new("foo".to_string()))));
                mock
            }
        }

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Provider user not found.(id: foo)".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_invalidalidated_key() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify()
                    .returning(|_| Err(VerifyError::InvalidatedApiKey));
                mock
            }
        }

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Invalidated api key".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_already_exist() {
        struct UC();
        impl SignUpUseCase for UC {}
        impl HaveUserRepository for UC {
            type UserRepository = MockUserRepository;
            fn user_repository(&self) -> Self::UserRepository {
                let mut mock = MockUserRepository::new();
                mock.expect_resolve()
                    .returning(|_| Ok(Some(User::new(UserId::new("foo".to_string())))));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveFirebaseAuthDriver for UC {
            type FirebaseAuthDriver = MockFirebaseAuthDriver;
            fn firebase_auth(&self) -> Self::FirebaseAuthDriver {
                let mut mock = MockFirebaseAuthDriver::new();
                mock.expect_verify().returning(|_| {
                    Ok(VerifyResult::new(
                        LocalId::new("foo".to_string()),
                        FullName::new("foo".to_string()),
                    ))
                });
                mock
            }
        }

        let usecase_result = UC().execute("xxxx".to_string()).await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "User is already exist. (id: foo)".to_string()
        );
    }
}
