use async_trait::async_trait;
use derive_more::{Constructor, Deref};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SignInWithIdpError {
    #[error("id: {id} entity is not exist")]
    AlreadyExist { id: String },
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

#[derive(Debug, Constructor, Clone)]
pub struct SignInWithIdpConfig {
    pub key: String,
    pub callback_url: String,
    pub provider_id_token: String,
    pub provider_id: String,
}

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct LocalId(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct FederatedId(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct FullName(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct AccessToken(String);

#[derive(Debug, Constructor, Clone, PartialEq, Eq, Deref)]
pub struct RefreashToken(String);

#[derive(Debug, Constructor, Clone)]
pub struct SignInWithIdpResult {
    pub local_id: LocalId,
    pub federated_id: FederatedId,
    pub full_name: FullName,
    pub access_token: AccessToken,
    pub refresh_token: RefreashToken,
}

#[async_trait]
trait FirebaseAuthDriver {
    async fn sign_in_with_idp(
        config: SignInWithIdpConfig,
    ) -> Result<SignInWithIdpResult, SignInWithIdpError>;
}
