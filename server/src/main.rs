use axum::{routing::get, Json};
use reqwest::StatusCode;
use serde_json::{json, Value};
use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::fmt::time::ChronoLocal;

#[tokio::main]
async fn main() {
    // init env
    dotenvy::dotenv().ok();

    // log
    let timer = ChronoLocal::new("%Y-%m-%d %H:%M:%S.%3f".into());
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_ansi(true)
        .with_line_number(true)
        .with_timer(timer)
        .init();

    let app = axum::Router::new().route("/:id", get(resp_json));

    let server = tokio::net::TcpListener::bind(std::env::var("ADDR").unwrap())
        .await
        .expect("Failed to bind address");

    let address = server.local_addr().unwrap();
    info!("Server started on {}", address);
    axum::serve(server, app)
        .await
        .expect("Failed to start server");
}

async fn resp_json(id: axum::extract::Path<String>) -> (StatusCode, Json<Value>) {
    let __id: isize;
    match id.parse::<isize>() {
        Ok(res) => __id = res,
        Err(_) => __id = 0,
    }
    let code: StatusCode;
    if __id > 10 {
        code = StatusCode::FORBIDDEN;
    } else {
        code = StatusCode::OK;
    }
    warn!("id: {}", __id);
    trace!("id: {}", __id);
    error!("id: {}", __id);
    debug!("id: {}", __id);
    (
        code,
        Json(json!({
            "id": __id,
            "message": "Hello, World!"
        })),
    )
}
