use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use derive_more::Constructor;
use serde::Serialize;
use serde_json::json;
use strum_macros::{Display, EnumString};
use tracing::error;

#[derive(EnumString, Debug, Display, Serialize, Clone)]
pub enum BadRequestKind {
    #[strum(serialize = "token_expired")]
    TokenExpired,
    #[strum(serialize = "verify_failed")]
    VerifyFailed,
    #[strum(serialize = "already_exist")]
    AlreadyExist,
    #[strum(serialize = "user_not_found")]
    UserNotFound,
}

#[derive(Debug, Clone, Serialize, Constructor)]
pub struct BadRequestPayload {
    kind: BadRequestKind,
    key: String,
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("bad request")]
    BadRequest(BadRequestPayload),
    #[error("authentication required")]
    Unauthorized,
    #[error("user may not perform that action")]
    Forbidden,
    #[error("request path not found")]
    NotFound,
    #[error("internal server error")]
    InternalServerErrorEmpty,
    #[error("internal server error")]
    InternalServerError(#[from] anyhow::Error),
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::InternalServerErrorEmpty => StatusCode::INTERNAL_SERVER_ERROR,
            Self::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Self::Unauthorized => {
                return (self.status_code(), Json(json!({"message": "unauthorized"})))
                    .into_response()
            }
            Self::InternalServerError(ref e) => {
                error!("Generic error: {:?}", e);
                (
                    self.status_code(),
                    Json(json!({"message": "internal server error"})),
                )
                    .into_response()
            }
            Self::BadRequest(ref payload) => {
                error!("Bad request: {:?}", payload);
                (self.status_code(), Json(payload)).into_response()
            }
            _ => (self.status_code(), Json(json!({"message": "other"}))).into_response(),
        }
    }
}
