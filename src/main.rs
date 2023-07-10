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
use rusqlite::Connection;

mod root;
mod meta;

use crate::root::root;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let _conn = connect_db().await.expect("couldn't connect to db");

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

async fn connect_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let conn = Connection::open("../data.sqlite")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS urls (
             id INTEGER PRIMARY KEY,
             url TEXT NOT NULL,
             hash TEXT NOT NULL,
             used_count INTEGER NOT NULL
         )",
        (),
    )?;
    return Ok(conn);
}
