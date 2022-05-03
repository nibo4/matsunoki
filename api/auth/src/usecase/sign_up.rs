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
