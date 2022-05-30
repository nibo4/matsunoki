use account::usecase::resolve_profile::{
    ResolveProfileUseCase, ResolveProfileUseCaseError, ResolveProfileUseCaseResult,
};
use anyhow::anyhow;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Extension;
use axum::Json;
use derive_more::Constructor;
use serde::Serialize;

use crate::actor::UserActor;
use crate::error::Error;
use crate::kernel::Kernel;

#[derive(Serialize, Constructor)]
pub struct ResolveProfileResponse(ResolveProfileUseCaseResult);

#[tracing::instrument(skip(kernel))]
pub async fn resolve_profile_handler(
    user_actor: UserActor,
    kernel: Extension<Kernel>,
) -> Result<Response, Error> {
    match kernel.execute(&user_actor).await {
        Ok(result) => Ok((StatusCode::OK, Json(ResolveProfileResponse(result))).into_response()),
        Err(e) => Err(match e {
            ResolveProfileUseCaseError::ResolveError(e) => Error::InternalServerError(anyhow!(e)),
            ResolveProfileUseCaseError::NotFound => Error::NotFound,
        }),
    }
}
