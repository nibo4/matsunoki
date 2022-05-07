use anyhow::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use account::adapter::firebase_auth::FullName;
use account::adapter::firebase_auth::LocalId;
use account::adapter::firebase_auth::{AccessToken, FirebaseAuthDriver, VerifyError, VerifyResult};
use account::effect::config::{Config, HaveConfig};
use derive_more::Constructor;
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use reqwest::get;
use tracing::info;

use crate::cache::{Cache, HaveCache};
use crate::config::DefaultConfig;

use std::collections::HashMap;

#[derive(Debug, Constructor)]
pub struct DefaultFirebaseAuthAdapter<T: Config>(T, Cache<JwkSet>);

impl HaveConfig for DefaultFirebaseAuthAdapter<DefaultConfig> {
    type Config = DefaultConfig;
    fn config(&self) -> &Self::Config {
        &self.0
    }
}

impl HaveCache<JwkSet> for DefaultFirebaseAuthAdapter<DefaultConfig> {
    fn cache(&self) -> &Cache<JwkSet> {
        &self.1
    }
}

#[async_trait]
impl FirebaseAuthDriver for DefaultFirebaseAuthAdapter<DefaultConfig> {
    #[tracing::instrument(skip(token, self))]
    async fn verify(&self, token: AccessToken) -> Result<VerifyResult, VerifyError> {
        let is_exist = self
            .cache()
            .lock()
            .map_err(|_| VerifyError::GetCacheStoreLockError)?
            .contains_key("jwks");
        if !is_exist {
            let jwks = get("https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com")
                .await
                .with_context(|| VerifyError::GetSecurityTokenError)?
                .json::<JwkSet>()
                .await
                .with_context(|| VerifyError::SecurityTokenDeserializeError)?;
            self.cache()
                .lock()
                .map_err(|_| VerifyError::GetCacheStoreLockError)?
                .insert("jwks".to_string(), jwks);
            info!("Writed jwks in memory-cache ");
        }

        let jwks = match self
            .cache()
            .lock()
            .map_err(|_| VerifyError::GetCacheStoreLockError)?
            .get("jwks")
        {
            Some(j) => j.clone(),
            None => {
                return Err(VerifyError::Unexpected(anyhow::anyhow!(
                    "jwks is not found in cache"
                )))
            }
        };

        let header = decode_header(&token).with_context(|| VerifyError::TokenHeaderDecodeError)?;
        let kid = match header.kid {
            Some(kid) => kid,
            None => {
                return Err(VerifyError::Unexpected(anyhow!(
                    "Token is not included kid"
                )))
            }
        };
        if let Some(j) = jwks.find(&kid) {
            match j.algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e)
                        .context(VerifyError::DecodeError)?;
                    let mut validation =
                        Validation::new(j.common.algorithm.context(VerifyError::Unexpected(
                            anyhow!("Algorithm is not found."),
                        ))?);
                    validation.set_audience(&[self.config().firebase_project_id().to_string()]);
                    let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                        &token,
                        &decoding_key,
                        &validation,
                    )
                    .with_context(|| VerifyError::DecodeError)?;
                    let uid = match decoded_token
                        .claims
                        .get("user_id")
                        .context(VerifyError::IdentifyNotFoundError)?
                        .as_str()
                    {
                        Some(v) => v,
                        None => return Err(VerifyError::IdentifyNotFoundError),
                    };
                    let name = match decoded_token
                        .claims
                        .get("name")
                        .context(VerifyError::IdentifyNotFoundError)?
                        .as_str()
                    {
                        Some(v) => v,
                        None => return Err(VerifyError::IdentifyNotFoundError),
                    };
                    Ok(VerifyResult::new(
                        LocalId::new(uid.to_string()),
                        FullName(name.to_string()),
                    ))
                }
                _ => Err(VerifyError::Unexpected(anyhow!("Unsupported algorithm"))),
            }
        } else {
            Err(VerifyError::Unexpected(anyhow!(
                "No matching JWK found for the given kid"
            )))
        }
    }
}
