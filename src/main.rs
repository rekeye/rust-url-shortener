use axum::{
    Router,
    http::StatusCode,
    routing::get,
    response::{
        Response,
        IntoResponse,
    }
};
use std::net::SocketAddr;

mod root;
mod meta;

use crate::root::root;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/styles.css", get(styles));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn styles() -> impl IntoResponse {
    return Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "text/css")
        .body(include_str!("../style/output.css").to_owned())
        .unwrap();
}
