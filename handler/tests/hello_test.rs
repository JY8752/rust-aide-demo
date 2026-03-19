mod helper;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;

#[tokio::test]
async fn test_hello() {
    let app = helper::setup_router().await;

    let response = app
        .oneshot(Request::get("/hello").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    helper::assert_text_response(response, "Hello, World!").await;
}
