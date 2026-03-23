#![allow(dead_code)]

use std::sync::Arc;

use app_state::AppState;
use axum::{
    Router,
    body::{Body, to_bytes},
    http::Request,
    response::Response,
};
use domain::user::repository::UserRepository as UserRepositoryPort;
use handler::{error::ErrorResponse, user::UserResponse};
use infrastructure::{db::user::UserRepository, slack::SlackClient};
use serde::{Serialize, de::DeserializeOwned};
use sqlx::PgPool;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{core::ContainerAsync, runners::AsyncRunner},
};
use tower::ServiceExt;
use usecase::{notify::Notifier, user::UserUseCase};
use uuid::Uuid;

pub struct TestApp {
    pub app: Router,
    _postgres: ContainerAsync<Postgres>,
}

pub struct SetupAppOptions {
    notifier: Arc<dyn Notifier + Send + Sync>,
}

impl Default for SetupAppOptions {
    fn default() -> Self {
        Self {
            notifier: Arc::new(SlackClient::new(
                std::env::var("SLACK_WEBHOOK_URL").unwrap_or_default(),
            )),
        }
    }
}

impl SetupAppOptions {
    pub fn with_notifier(mut self, notifier: Arc<dyn Notifier + Send + Sync>) -> Self {
        self.notifier = notifier;
        self
    }
}

impl TestApp {
    pub async fn request(&self, request: Request<Body>) -> Response {
        self.app.clone().oneshot(request).await.unwrap()
    }

    pub async fn get(&self, path: &str) -> Response {
        self.request(Request::get(path).body(Body::empty()).unwrap())
            .await
    }

    pub async fn post_json<T>(&self, path: &str, body: &T) -> Response
    where
        T: Serialize,
    {
        let body = serde_json::to_vec(body).unwrap();

        self.request(
            Request::post(path)
                .header("content-type", "application/json")
                .body(Body::from(body))
                .unwrap(),
        )
        .await
    }
}

pub async fn setup_app(options: SetupAppOptions) -> TestApp {
    let postgres = Postgres::default()
        .with_db_name("app")
        .with_user("postgres")
        .with_password("pass")
        .with_init_sql(include_str!("../../../schema.sql").as_bytes().to_vec())
        .start()
        .await
        .unwrap();

    let host = postgres.get_host().await.unwrap();
    let port = postgres.get_host_port_ipv4(5432).await.unwrap();
    let database_url = format!("postgres://postgres:pass@{host}:{port}/app?sslmode=disable");
    let pool = PgPool::connect(&database_url).await.unwrap();

    let user_repository: Arc<dyn UserRepositoryPort + Send + Sync> =
        Arc::new(UserRepository::new(pool));
    let user_usecase = UserUseCase::new(user_repository, options.notifier);

    let state = AppState::new(user_usecase);

    TestApp {
        app: handler::router(state),
        _postgres: postgres,
    }
}

pub async fn assert_text_response(response: Response, expected: &str) {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, expected);
}

pub async fn from_json_response<T>(response: Response) -> T
where
    T: DeserializeOwned,
{
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body).unwrap()
}

pub fn assert_created_user_response(
    user: &UserResponse,
    expected_name: &str,
    expected_email: &str,
) {
    assert!(Uuid::parse_str(&user.id).is_ok());

    let expected = UserResponse {
        id: user.id.clone(),
        name: expected_name.to_string(),
        email: expected_email.to_string(),
    };

    assert_eq!(user, &expected);
}

pub fn assert_error_response(error: &ErrorResponse, expected_message: &str) {
    let expected = ErrorResponse {
        message: expected_message.to_string(),
    };

    assert_eq!(error, &expected);
}
