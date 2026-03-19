use app_state::AppState;
use axum::{Router, body::to_bytes, response::Response};
use sqlx::PgPool;

pub async fn setup_router() -> Router {
    let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:pass@localhost:5432/app?sslmode=disable".to_string()
    });
    let pool = PgPool::connect(&database_url).await.unwrap();
    let state = AppState::from_pool(pool);
    handler::router(state)
}

pub async fn assert_text_response(response: Response, expected: &str) {
    let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(body, expected);
}
