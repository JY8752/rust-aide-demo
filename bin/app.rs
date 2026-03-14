use anyhow::Context;
use app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};
use handler::{hello, user};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    tracing::info!("{}", std::env::var("HELLO")?);

    let database_url = std::env::var("DATABASE_URL").context("DATABASE_URL is not set")?;
    let state = AppState::from_database_url(&database_url).await?;

    let app = Router::new()
        .route("/hello", get(hello::hello))
        .route("/users/{id}", get(user::get_user))
        .route("/users", post(user::create_user))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    let local_addr = listener.local_addr()?;
    tracing::info!(%local_addr, "listening");

    axum::serve(listener, app).await?;
    Ok(())
}

fn init_tracing() {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let json_log = std::env::var("LOG_FORMAT")
        .map(|v| v.eq_ignore_ascii_case("json"))
        .unwrap_or(false);

    if json_log {
        tracing_subscriber::fmt()
            .with_env_filter(env_filter)
            .json()
            .init();
        return;
    }

    tracing_subscriber::fmt().with_env_filter(env_filter).init();
}
