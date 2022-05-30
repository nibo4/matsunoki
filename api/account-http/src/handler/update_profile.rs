use account::usecase::update_profile::{
    UpdateProfileUseCase, UpdateProfileUseCaseError, UpdateProfileUseCaseParams,
    UpdateProfileUseCaseResult,
};
use anyhow::anyhow;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

use crate::actor::UserActor;
use crate::error::Error;
use crate::error::{BadRequestKind, BadRequestPayload};
use crate::kernel::Kernel;

#[derive(Serialize, Constructor)]
pub struct UpdateProfileResponse(UpdateProfileUseCaseResult);

#[derive(Serialize, Deserialize, Constructor)]
pub struct UpdateProfileParams(UpdateProfileUseCaseParams);

#[tracing::instrument(skip(kernel))]
pub async fn update_profile_handler(
    user_actor: UserActor,
    kernel: Extension<Kernel>,
    params: UpdateProfileUseCaseParams,
) -> Result<Response, Error> {
    match kernel.execute(&user_actor, params).await {
        Ok(result) => Ok((StatusCode::OK, Json(UpdateProfileResponse(result))).into_response()),
        Err(e) => Err(match e {
            UpdateProfileUseCaseError::ProfileValidationError(e) => Error::BadRequest(
                BadRequestPayload::new(BadRequestKind::ProfileValidationError, format!("{:?}", e)),
            ),
            UpdateProfileUseCaseError::StoreError(e) => Error::InternalServerError(anyhow!(e)),
        }),
    }
}
