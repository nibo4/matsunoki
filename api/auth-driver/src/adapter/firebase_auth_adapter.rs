use anyhow::anyhow;
use anyhow::Context;
use async_trait::async_trait;
use auth::adapter::firebase_auth::{AccessToken, FirebaseAuthDriver, VerifyError, VerifyResult};
use auth::effect::config::{Config, HaveConfig};
use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, DecodingKey, Validation};
use reqwest::get;

use crate::config::DefaultConfig;

use std::collections::HashMap;

pub struct DefaultFirebaseAuthAdapter<T: Config>(T);

impl HaveConfig for DefaultFirebaseAuthAdapter<DefaultConfig> {
    type Config = DefaultConfig;
    fn config(&self) -> &Self::Config {
        &self.0
    }
}

#[async_trait]
impl FirebaseAuthDriver for DefaultFirebaseAuthAdapter<DefaultConfig> {
    async fn verify(&self, token: AccessToken) -> Result<VerifyResult, VerifyError> {
        let jwks = get("https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com")
            .await
            .with_context(|| VerifyError::GetSecurityTokenError)?
            .json::<JwkSet>()
            .await
            .with_context(|| VerifyError::SecurityTokenDeserializeError)?;
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
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                    let mut validation = Validation::new(j.common.algorithm.unwrap());
                    validation.validate_exp = false;
                    let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                        &token,
                        &decoding_key,
                        &validation,
                    )
                    .unwrap();
                    println!("{:?}", decoded_token);
                    unimplemented!()
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
