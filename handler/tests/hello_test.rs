mod helper;

use axum::http::StatusCode;

#[tokio::test]
async fn test_hello() {
    let test_app = helper::setup_app().await;

    let response = test_app.get("/hello").await;

    assert_eq!(response.status(), StatusCode::OK);
    helper::assert_text_response(response, "Hello, World!").await;
}
