mod api;
mod routes;
mod components;

use axum::{
    Router,
    http::StatusCode,
    routing::{get,post},
    response::{
        Response,
        IntoResponse,
    }
};
use std::net::SocketAddr;

use crate::{
    routes::root::root,
    api::connect_db,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _conn = connect_db().await.expect("couldn't connect to db");

    let app = Router::new()
        .route("/", get(root))
        .route("/styles.css", get(styles))
        .route("/api/*fn_name", post(leptos_axum::handle_server_fns));

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

