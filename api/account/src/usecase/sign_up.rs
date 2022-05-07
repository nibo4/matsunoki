#[cfg(test)]
use crate::adapter::firebase_auth::MockFirebaseAuthDriver;
use crate::adapter::firebase_auth::{
    AccessToken, FirebaseAuthDriver, HaveFirebaseAuthDriver, VerifyError,
};
#[cfg(test)]
use crate::effect::id_generator::MockIdGenerator;
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
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Constructor, Serialize)]
pub struct SignUpUseCaseResult {
    user_id: UserId,
    name: String,
}

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

        self.user_repository().store(&sign_up_user).await?;
        Ok(SignUpUseCaseResult::new(
            sign_up_user.id,
            verify_result.full_name.0,
        ))
    }
}
impl<T: HaveUserRepository + HaveFirebaseAuthDriver + HaveIdGenerator> SignUpUseCase for T {}

#[cfg(test)]
mockall::mock! {
    pub SignUpUseCase {}

    impl HaveUserRepository for SignUpUseCase {
        type UserRepository = MockUserRepository;
        fn user_repository(&self) -> &MockUserRepository;
    }

    impl HaveFirebaseAuthDriver for SignUpUseCase {
        type FirebaseAuthDriver = MockFirebaseAuthDriver;
        fn firebase_auth(&self) -> &MockFirebaseAuthDriver;
    }

    impl HaveIdGenerator for SignUpUseCase {
        type IdGenerator = MockIdGenerator;
        fn id_generator(&self) -> &MockIdGenerator;
    }
}

#[cfg(test)]
mod tests {
    use super::SignUpUseCase;
    use crate::adapter::firebase_auth::{
        FullName, HaveFirebaseAuthDriver, LocalId, MockFirebaseAuthDriver, VerifyError,
        VerifyResult,
    };
    use crate::effect::id_generator::{HaveIdGenerator, MockIdGenerator};
    use crate::model::user::{User, UserId};
    use crate::repository::user_repository::{HaveUserRepository, MockUserRepository};

    use derive_more::Constructor;

    #[derive(Constructor)]
    struct UC {
        user_repo: MockUserRepository,
        firebase_auth: MockFirebaseAuthDriver,
        id_gen: MockIdGenerator,
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

    impl HaveIdGenerator for UC {
        type IdGenerator = MockIdGenerator;
        fn id_generator(&self) -> &MockIdGenerator {
            &self.id_gen
        }
    }

    #[tokio::test]
    async fn sign_up_return_ok_when_verify_ok_and_user_repository_return_empty() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth.expect_verify().returning(|_| {
            Ok(VerifyResult::new(
                LocalId::new("DUMMY".to_string()),
                FullName::new("FULL NAME".to_string()),
            ))
        });
        id_gen.expect_generate().returning(|| "xxxx".to_string());
        assert!(UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await
            .is_ok())
    }

    #[tokio::test]
    async fn sign_up_return_err_when_token_expire() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth
            .expect_verify()
            .returning(|_| Err(VerifyError::TokenExpired));
        id_gen.expect_generate().returning(|| "xxxx".to_string());
        let usecase_result = UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Token expired.".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_user_disabled() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth
            .expect_verify()
            .returning(|_| Err(VerifyError::UserDisabled(LocalId::new("foo".to_string()))));
        id_gen.expect_generate().returning(|| "xxxx".to_string());

        let usecase_result = UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Provider user disabled.(id: foo)".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_user_not_found() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth
            .expect_verify()
            .returning(|_| Err(VerifyError::UserNotFound(LocalId::new("foo".to_string()))));
        id_gen.expect_generate().returning(|| "xxxx".to_string());

        let usecase_result = UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Provider user not found.(id: foo)".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_invalidalidated_key() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(None));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth
            .expect_verify()
            .returning(|_| Err(VerifyError::InvalidatedApiKey));
        id_gen.expect_generate().returning(|| "xxxx".to_string());

        let usecase_result = UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "Invalidated api key".to_string()
        );
    }

    #[tokio::test]
    async fn sign_up_return_err_when_already_exist() {
        let mut user_repo = MockUserRepository::new();
        let mut firebase_auth = MockFirebaseAuthDriver::new();
        let mut id_gen = MockIdGenerator::new();

        user_repo
            .expect_find_by_id_in_provider()
            .returning(|_| Ok(Some(User::default())));
        user_repo.expect_store().returning(|_| Ok(()));
        firebase_auth
            .expect_verify()
            .returning(|_| Ok(VerifyResult::default()));
        id_gen.expect_generate().returning(|| "xxxx".to_string());

        let usecase_result = UC::new(user_repo, firebase_auth, id_gen)
            .execute("xxxx".to_string())
            .await;
        assert!(usecase_result.is_err());
        assert_eq!(
            usecase_result.err().unwrap().to_string(),
            "User is already exist. (id: )".to_string()
        );
    }
}
