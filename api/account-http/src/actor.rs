use account::actor::user::User;
use account::usecase::verify::VerifyUseCase;
use anyhow::anyhow;
use async_trait::async_trait;
use axum::extract::FromRequest;
use axum::extract::RequestParts;
use axum::headers;
use axum::headers::authorization::Bearer;
use axum::BoxError;
use axum::Extension;
use axum::TypedHeader;
use derive_more::Deref;
use http_body::Body;

use crate::error::Error;
use crate::kernel::Kernel;

#[derive(Debug, Clone, Deref)]
pub struct UserActor(User);

#[async_trait]
impl<B> FromRequest<B> for UserActor
where
    B: Body + Send,
    B::Data: Send,
    B::Error: Into<BoxError>,
{
    type Rejection = Error;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let authorization = req
            .extract::<TypedHeader<headers::Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Unauthorized)?;
        let kernel = req
            .extract::<Extension<Kernel>>()
            .await
            .map_err(|_| Error::InternalServerError(anyhow!("kernel is not provided")))?;
        let verify_result = kernel
            .execute(authorization.token())
            .await
            .map_err(|_| Error::Unauthorized)?;
        Ok(UserActor(verify_result.user.into()))
    }
}
