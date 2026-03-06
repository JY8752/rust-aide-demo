use axum::{Router, routing::get};
use handler::hello;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/hello", get(hello::hello));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    let local_addr = listener.local_addr()?;
    println!("Listening on {}", local_addr);

    axum::serve(listener, app).await?;
    Ok(())
}
