use aide::transform::TransformOperation;
use app_state::AppState;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use domain::user::UserId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiError, ErrorResponse, map_application_error};

#[derive(Deserialize, JsonSchema)]
pub struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Deserialize, JsonSchema)]
pub struct GetUserPath {
    id: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, JsonSchema, Deserialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl From<&domain::user::User> for UserResponse {
    fn from(user: &domain::user::User) -> Self {
        let id: Uuid = user.id().clone().into();
        let name: String = user.name().clone().into();
        let email: String = user.email().clone().into();

        Self {
            id: id.to_string(),
            name,
            email,
        }
    }
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), ApiError> {
    match state
        .user_usecase()
        .create_user(&payload.name, &payload.email)
        .await
    {
        Ok(user) => Ok((StatusCode::CREATED, Json(UserResponse::from(&user)))),
        Err(err) => Err(map_application_error(err)),
    }
}

pub async fn get_user(
    Path(path): Path<GetUserPath>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, ApiError> {
    let uuid = match Uuid::parse_str(&path.id) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::invalid_user_id()),
    };

    let user_id = match UserId::new(uuid) {
        Ok(id) => id,
        Err(_) => return Err(ApiError::invalid_user_id()),
    };

    match state.user_usecase().find_user_by_id(&user_id).await {
        Ok(Some(user)) => Ok(Json(UserResponse::from(&user))),
        Ok(None) => Err(ApiError::user_not_found()),
        Err(err) => Err(map_application_error(err)),
    }
}

pub fn create_user_docs(op: TransformOperation<'_>) -> TransformOperation<'_> {
    op.summary("Create user")
        .description("Create a new user with name and email.")
        .response_with::<201, Json<UserResponse>, _>(|res| res.description("User created"))
        .response_with::<400, Json<ErrorResponse>, _>(|res| res.description("Invalid request"))
        .response_with::<500, Json<ErrorResponse>, _>(|res| {
            res.description("Internal server error")
        })
}

// pub fn get_user_docs(op: TransformOperation<'_>) -> TransformOperation<'_> {
//     op.summary("Get user")
//         .description("Fetch a user by ID.")
//         .response_with::<200, Json<UserResponse>, _>(|res| res.description("User found"))
//         .response_with::<400, Json<ErrorResponse>, _>(|res| res.description("Invalid user ID"))
//         .response_with::<404, Json<ErrorResponse>, _>(|res| res.description("User not found"))
//         .response_with::<500, Json<ErrorResponse>, _>(|res| {
//             res.description("Internal server error")
//         })
// }
