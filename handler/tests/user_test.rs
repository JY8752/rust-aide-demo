mod helper;

use axum::http::StatusCode;
use handler::error::ErrorResponse;
use handler::user::UserResponse;
use mockall::{mock, predicate};
use serde::Serialize;
use std::sync::Arc;
use usecase::{error::ApplicationError, notify::Notifier};

#[derive(Serialize)]
struct CreateUserRequest {
    name: String,
    email: String,
}

mock! {
    Notifier {}

    impl Notifier for Notifier {
        fn notify(&self, message: &str) -> Result<(), ApplicationError>;
    }
}

#[tokio::test]
async fn test_create_user() {
    let test_app = helper::setup_app(helper::SetupAppOptions::default()).await;
    let request = CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let response = test_app.post_json("/users", &request).await;

    assert_eq!(response.status(), StatusCode::CREATED);
    let user: UserResponse = helper::from_json_response(response).await;
    helper::assert_created_user_response(&user, "Alice", "alice@example.com");
}

#[tokio::test]
async fn test_create_user_notifies_on_success() {
    // Arrange
    let mut notifier = MockNotifier::new();
    notifier
        .expect_notify()
        .with(predicate::str::contains("alice@example.com"))
        .times(1)
        .returning(|_| Ok(()));

    let test_app =
        helper::setup_app(helper::SetupAppOptions::default().with_notifier(Arc::new(notifier)))
            .await;
    let request = CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Act
    let response = test_app.post_json("/users", &request).await;

    // Assert
    assert_eq!(response.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_get_user() {
    // Arrange
    let test_app = helper::setup_app(helper::SetupAppOptions::default()).await;
    let request = CreateUserRequest {
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    let created = test_app.post_json("/users", &request).await;

    assert_eq!(created.status(), StatusCode::CREATED);
    let created: UserResponse = helper::from_json_response(created).await;
    let created_id = created.id.clone();

    let expected = UserResponse {
        id: created_id.clone(),
        name: "Alice".to_string(),
        email: "alice@example.com".to_string(),
    };

    // Act
    let act = test_app.get(&format!("/users/{}", created_id)).await;

    // Assert
    assert_eq!(act.status(), StatusCode::OK);
    let act: UserResponse = helper::from_json_response(act).await;
    assert_eq!(act, expected);
}

#[tokio::test]
async fn test_create_user_returns_bad_request_for_invalid_email() {
    let mut notifier = MockNotifier::new();
    notifier.expect_notify().times(0);

    let test_app =
        helper::setup_app(helper::SetupAppOptions::default().with_notifier(Arc::new(notifier)))
            .await;
    let request = CreateUserRequest {
        name: "Alice".to_string(),
        email: "invalid-email".to_string(),
    };

    let response = test_app.post_json("/users", &request).await;

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let error: ErrorResponse = helper::from_json_response(response).await;
    helper::assert_error_response(&error, "invalid request");
}
