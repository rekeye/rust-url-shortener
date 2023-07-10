use rusqlite::{Connection, params};
use leptos::{server, ServerFnError};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,)]
struct Url {
    id: u16,
    url: String,
    hash: String,
    used_count: u16
}

pub async fn connect_db() -> Result<Connection, ServerFnError> {
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

#[server(GetUrl, "/api", "GetJson")]
pub async fn get_url(hash: String) -> Result<String, ServerFnError> {
    let connection = connect_db().await?;
    
    let query = "SELECT * FROM urls WHERE hash = ?";
    let mut statement = connection.prepare(query)?;
    let mut rows = statement.query(params![hash])?;
    let first_row = rows.next()?;
    let url: String = first_row.expect("url not found").get(1)?;

    return Ok(url);
}

#[server(GenerateUrl, "/api")]
pub async fn generate_url(input_url: String) -> Result<String, ServerFnError> {
    // TODO: generate the hash
    let hash = "";

    let connection = connect_db().await?;
    
    let query = "INSERT INTO users (url, hash, used_count) VALUES (?, ?, 0)";
    let mut statement = connection.prepare(query)?;
    let mut rows = statement.query(params![input_url, hash])?;
    let first_row = rows.next()?;
    let url: String = first_row.expect("url not found").get(1)?;

    return Ok(url);
}

