use async_trait::async_trait;
use derive_more::{Constructor, Deref, Display};
use thiserror::Error;

use crate::effect::config::{Config, HaveConfig};

#[cfg(test)]
use crate::effect::config::MockConfig;

#[derive(Error, Debug)]
pub enum VerifyError {
    #[error("Token expired.")]
    TokenExpired,
    #[error("Provider user disabled.(id: {0})")]
    UserDisabled(LocalId),
    #[error("Provider user not found.(id: {0})")]
    UserNotFound(LocalId),
    #[error("Invalidated api key")]
    InvalidatedApiKey,
    #[error("Get security token error")]
    GetSecurityTokenError,
    #[error("Security token deserialize error")]
    SecurityTokenDeserializeError,
    #[error("Token header decode error")]
    TokenHeaderDecodeError,
    #[error("Token decode error")]
    DecodeError,
    #[error("Identify infomation not found")]
    IdentifyNotFoundError,
    #[error("Failed get cache store lock")]
    GetCacheStoreLockError,
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Constructor, Clone, Default)]
pub struct VerifyResult {
    pub uid: LocalId,
    pub full_name: FullName,
}

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display, Default)]
pub struct LocalId(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display, Default)]
pub struct FederatedId(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref, Display, Default)]
pub struct FullName(pub String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct AccessToken(pub String);

#[async_trait]
pub trait FirebaseAuthDriver: HaveConfig {
    async fn verify(&self, token: AccessToken) -> Result<VerifyResult, VerifyError>;
}

#[cfg(test)]
mockall::mock! {
    pub FirebaseAuthDriver {}

    impl HaveConfig for FirebaseAuthDriver {
        type Config = MockConfig;
        fn config(&self) -> &MockConfig;
    }

    #[async_trait]
    impl FirebaseAuthDriver for FirebaseAuthDriver {
        async fn verify(&self, token: AccessToken) -> Result<VerifyResult, VerifyError>;
    }
}

#[cfg_attr(test, mockall::automock(type FirebaseAuthDriver = MockFirebaseAuthDriver;))]
pub trait HaveFirebaseAuthDriver {
    type FirebaseAuthDriver: FirebaseAuthDriver + Send + Sync + 'static;
    fn firebase_auth(&self) -> &Self::FirebaseAuthDriver;
}
