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
use tower_http::services::ServeDir;
use std::net::SocketAddr;

use crate::{
    routes::root::root,
    api::{
        connect_db,
        redirect,
        create_hash
    }
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _conn = connect_db().await.expect("couldn't connect to db");

    let serve_dir = ServeDir::new("assets");

    let app = Router::new()
        .route("/", get(root))
        .route("/:hash", get(redirect))
        .route("/styles.css", get(styles))
        .route("/api/create-hash", post(create_hash))
        .nest_service("/assets", serve_dir.clone());

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
        .body(load_file::load_str!("../style/output.css").to_owned())
        .unwrap();
}

