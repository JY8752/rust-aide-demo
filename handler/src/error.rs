use axum::{Json, http::StatusCode, response::IntoResponse};
use schemars::JsonSchema;
use serde::Serialize;
use usecase::error::ApplicationError;

#[derive(Serialize, JsonSchema)]
pub struct ErrorResponse {
    message: &'static str,
}

pub struct ApiError {
    status: StatusCode,
    message: &'static str,
}

impl ApiError {
    pub fn new(status: StatusCode, message: &'static str) -> Self {
        Self { status, message }
    }

    pub fn invalid_user_id() -> Self {
        Self::new(StatusCode::BAD_REQUEST, "invalid user id")
    }

    pub fn user_not_found() -> Self {
        Self::new(StatusCode::NOT_FOUND, "user not found")
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        (
            self.status,
            Json(ErrorResponse {
                message: self.message,
            }),
        )
            .into_response()
    }
}

impl aide::OperationOutput for ApiError {
    type Inner = ErrorResponse;
}

pub fn map_application_error(err: ApplicationError) -> ApiError {
    match err {
        ApplicationError::DomainError(err) => {
            tracing::warn!(error = %err, "domain error");
            ApiError::new(StatusCode::BAD_REQUEST, "invalid request")
        }
        ApplicationError::SystemError(err) => {
            tracing::error!(error = %err, "system error");
            ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        }
    }
}
