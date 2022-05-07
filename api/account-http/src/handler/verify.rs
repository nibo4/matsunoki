use account::usecase::verify::{VerifyUseCase, VerifyUseCaseError, VerifyUseCaseResult};
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
pub struct VerifyResponse(VerifyUseCaseResult);

#[tracing::instrument(skip(kernel, authorization))]
pub async fn verify_handler(
    kernel: Extension<Kernel>,
    TypedHeader(authorization): TypedHeader<headers::Authorization<Bearer>>,
) -> Result<Response, Error> {
    match kernel.execute(&authorization.token()).await {
        Ok(result) => Ok((StatusCode::CREATED, Json(VerifyResponse::new(result))).into_response()),
        Err(e) => Err(match e {
            VerifyUseCaseError::UserNotFound(e) => {
                Error::BadRequest(BadRequestPayload::new(BadRequestKind::UserNotFound, e))
            }
            VerifyUseCaseError::VerifyFailed(e) => Error::BadRequest(BadRequestPayload::new(
                BadRequestKind::VerifyFailed,
                e.to_string(),
            )),
            VerifyUseCaseError::FilterError(e) => Error::InternalServerError(anyhow!(e)),
        }),
    }
}
