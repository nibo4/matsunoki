#[cfg(test)]
use crate::driver::firebase_auth::MockFirebaseAuthDriver;
use crate::driver::firebase_auth::{
    AccessToken, FirebaseAuthDriver, HaveFirebaseAuthDriver, VerifyError,
};
use crate::effect::id_generator::{HaveIdGenerator, IdGenerator};
use crate::model::login_provider::{IdInProvider, LoginProvider, ProviderKind};
use crate::model::user::{User, UserId};
#[cfg(test)]
use crate::repository::user_repository::MockUserRepository;
use crate::repository::user_repository::{
    FilterByIdInProviderError, HaveUserRepository, StoreError, UserRepository,
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
    ResolveError(#[from] FilterByIdInProviderError),
    #[error(transparent)]
    StoreError(#[from] StoreError),
    #[error("User is already exist. (id: {0})")]
    AlreadyExist(String),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[async_trait]
pub trait SignUpUseCase: HaveUserRepository + HaveFirebaseAuthDriver + HaveIdGenerator {
    async fn execute(&self, token: String) -> Result<SignUpUseCaseResult, SignUpUseCaseError> {
        let verify_result = self.firebase_auth().verify(AccessToken::new(token)).await?;
        let id_in_provider = IdInProvider::new(verify_result.uid.0);

        if self
            .user_repository()
            .find_by_id_in_provider(&id_in_provider)
            .await?
            .is_some()
        {
            return Err(SignUpUseCaseError::AlreadyExist(id_in_provider.0.clone()));
        }

        let sign_up_user = User::new(
            UserId::new(self.id_generator().generate()),
            Some(vec![LoginProvider::new(
                ProviderKind::Google,
                id_in_provider,
            )]),
        );

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
        FullName, HaveFirebaseAuthDriver, LocalId, MockFirebaseAuthDriver, VerifyError,
        VerifyResult,
    };
    use crate::effect::id_generator::{HaveIdGenerator, MockIdGenerator};
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
                mock.expect_store().returning(|_| Ok(()));
                mock
            }
        }

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider().returning(|_| Ok(None));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;
            fn id_generator(&self) -> MockIdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
                mock.expect_find_by_id_in_provider()
                    .returning(|_| Ok(Some(User::new(UserId::new("foo".to_string()), None))));
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

        impl HaveIdGenerator for UC {
            type IdGenerator = MockIdGenerator;

            fn id_generator(&self) -> Self::IdGenerator {
                let mut mock = MockIdGenerator::new();
                mock.expect_generate().returning(|| "xxxxx".to_string());
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
