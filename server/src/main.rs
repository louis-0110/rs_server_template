use axum::{routing::get, Json};
use reqwest::StatusCode;
use serde_json::{json, Value};
use tracing::info;
use tracing_subscriber::fmt::time::ChronoLocal;

#[tokio::main]
async fn main() {
    // log
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S.%3f".into());
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_line_number(true)
        .with_timer(timer)
        .init();

    let app = axum::Router::new().route("/", get(resp_json));

    let server = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("Failed to bind address");
    info!("Server started on http://127.0.0.1:3000");
    axum::serve(server, app)
        .await
        .expect("Failed to start server");
}

async fn resp_json() -> (StatusCode, Json<Value>) {
    (
        StatusCode::FORBIDDEN,
        Json(json!({
            "status": "ok",
            "message": "Hello, World!"
        })),
    )
}
