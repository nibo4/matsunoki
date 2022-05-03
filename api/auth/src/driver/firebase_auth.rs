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
pub struct LocalId(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display)]
pub struct FederatedId(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display)]
pub struct FullName(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct AccessToken(String);

#[async_trait]
trait FirebaseAuthDriver {
    async fn verify(token: AccessToken) -> Result<VerifyResult, VerifyError>;
}
