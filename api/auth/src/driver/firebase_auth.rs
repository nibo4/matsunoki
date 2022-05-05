use async_trait::async_trait;
use derive_more::{Constructor, Deref, Display};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Token expired.")]
    TokenExpired,
    #[error("Provider user disabled.(id: ${0})")]
    UserDisabled(LocalId),
    #[error("Provider user not found.(id: ${0})")]
    UserNotFound(LocalId),
    #[error("Invalidated api key")]
    InvalidatedApiKey,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Constructor, Clone)]
pub struct VerifyResult {
    pub uid: LocalId,
    pub full_name: FullName,
}

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display)]
pub struct LocalId(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display)]
pub struct FederatedId(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display)]
pub struct FullName(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct AccessToken(pub String);

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait FirebaseAuthDriver {
    async fn verify(&self, token: AccessToken) -> Result<VerifyResult, VerifyError>;
}

#[cfg_attr(test, mockall::automock(type FirebaseAuthDriver = MockFirebaseAuthDriver;))]
pub trait HaveFirebaseAuthDriver {
    type FirebaseAuthDriver: FirebaseAuthDriver + Send + Sync + 'static;
    fn firebase_auth(&self) -> Self::FirebaseAuthDriver;
}
