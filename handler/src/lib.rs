pub mod error;
pub mod hello;
pub mod user;

use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, post_with},
    },
    openapi::{Info, OpenApi},
    swagger::Swagger,
};
use app_state::AppState;
use axum::{Extension, Json, Router};

pub fn router(state: AppState) -> Router {
    let mut api = OpenApi {
        info: Info {
            title: "rust-aide-demo".into(),
            description: Some("Sample API documented with aide.".into()),
            version: env!("CARGO_PKG_VERSION").into(),
            ..Info::default()
        },
        ..OpenApi::default()
    };

    ApiRouter::new()
        .route("/docs", Swagger::new("/openapi.json").axum_route())
        .route("/openapi.json", get(serve_api))
        .route("/hello", get(hello::hello))
        .api_route("/users/{id}", get(user::get_user))
        .api_route(
            "/users",
            post_with(user::create_user, user::create_user_docs),
        )
        .with_state(state)
        .finish_api(&mut api)
        .layer(Extension(api))
}

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
    Json(api)
}
