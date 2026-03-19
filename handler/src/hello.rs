pub async fn hello() -> &'static str {
    "Hello, World!"
}

#[cfg(test)]
mod tests {
    use app_state::AppState;
    use axum::{
        Router,
        body::{Body, to_bytes},
        http::{Request, StatusCode},
    };
    use sqlx::PgPool;
    use tower::ServiceExt;

    #[tokio::test]
    async fn helloハンドラーはhttpレスポンスとして固定メッセージを返す() {
        let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
            "postgres://postgres:pass@localhost:5432/app?sslmode=disable".to_string()
        });
        let pool = PgPool::connect(&database_url).await.unwrap();
        let state = AppState::from_pool(pool);
        let app: Router = crate::router(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/hello")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        assert_eq!(body, "Hello, World!");
    }
}
