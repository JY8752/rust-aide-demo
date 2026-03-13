use app_state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use domain::user::UserId;
use serde::Deserialize;
use usecase::error::ApplicationError;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> impl IntoResponse {
    match state
        .user_usecase()
        .create_user(&payload.name, &payload.email)
        .await
    {
        Ok(user) => {
            let id: Uuid = user.id().clone().into();
            let name: String = user.name().clone().into();
            let email: String = user.email().clone().into();

            (
                StatusCode::CREATED,
                format!("id={id}, name={name}, email={email}"),
            )
                .into_response()
        }
        Err(err) => map_application_error(err),
    }
}

pub async fn get_user(Path(id): Path<String>, State(state): State<AppState>) -> impl IntoResponse {
    let uuid = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid user id").into_response(),
    };

    let user_id = match UserId::new(uuid) {
        Ok(id) => id,
        Err(_) => return (StatusCode::BAD_REQUEST, "invalid user id").into_response(),
    };

    match state.user_usecase().find_user_by_id(&user_id).await {
        Ok(Some(user)) => {
            let id: Uuid = user.id().clone().into();
            let name: String = user.name().clone().into();
            let email: String = user.email().clone().into();

            (
                StatusCode::OK,
                format!("id={id}, name={name}, email={email}"),
            )
                .into_response()
        }
        Ok(None) => (StatusCode::NOT_FOUND, "user not found").into_response(),
        Err(err) => map_application_error(err),
    }
}

// とりあえず雑にハンドリング
// 重要なのはドメインエラーとそれ以外で分けてハンドリングできること
fn map_application_error(err: ApplicationError) -> axum::response::Response {
    match err {
        ApplicationError::DomainError(err) => {
            tracing::warn!(error = %err, "domain error");
            (StatusCode::BAD_REQUEST, "invalid request").into_response()
        }
        ApplicationError::SystemError(err) => {
            tracing::error!(error = %err, "system error");
            (StatusCode::INTERNAL_SERVER_ERROR, "internal server error").into_response()
        }
    }
}
