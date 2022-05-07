use account::usecase::sign_up::{SignUpUseCase, SignUpUseCaseError, SignUpUseCaseResult};
use anyhow::anyhow;
use axum::extract::TypedHeader;
use axum::headers;
use axum::headers::authorization::Bearer;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use derive_more::Constructor;
use serde::Serialize;

use crate::error::Error;
use crate::error::{BadRequestKind, BadRequestPayload};
use crate::kernel::Kernel;

#[derive(Serialize, Constructor)]
pub struct SignUpResponse(SignUpUseCaseResult);

#[tracing::instrument(skip(kernel))]
pub async fn sign_up_handler(
    kernel: Extension<Kernel>,
    TypedHeader(authorization): TypedHeader<headers::Authorization<Bearer>>,
) -> Result<Response, Error> {
    match kernel.execute(authorization.token().to_string()).await {
        Ok(result) => Ok((StatusCode::CREATED, Json(SignUpResponse::new(result))).into_response()),
        Err(e) => Err(match e {
            SignUpUseCaseError::AlreadyExist(e) => {
                Error::BadRequest(BadRequestPayload::new(BadRequestKind::AlreadyExist, e))
            }
            SignUpUseCaseError::VerifyFailed(e) => Error::BadRequest(BadRequestPayload::new(
                BadRequestKind::VerifyFailed,
                e.to_string(),
            )),
            SignUpUseCaseError::ResolveError(e) => Error::InternalServerError(anyhow!(e)),
            SignUpUseCaseError::StoreError(e) => Error::InternalServerError(anyhow!(e)),
            SignUpUseCaseError::Unexpected(e) => Error::InternalServerError(anyhow!(e)),
        }),
    }
}
