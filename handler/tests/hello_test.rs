mod helper;

use axum::http::StatusCode;

#[tokio::test]
async fn test_hello() {
    let test_app = helper::setup_app(helper::SetupAppOptions::default()).await;

    let response = test_app.get("/hello").await;

    assert_eq!(response.status(), StatusCode::OK);
    helper::assert_text_response(response, "Hello, World!").await;
}
