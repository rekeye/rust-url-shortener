use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Json}
};
use rusqlite::{Connection};
use serde::{Serialize, Deserialize};

const SELECT_QUERY: &'static str = "SELECT url FROM urls WHERE hash = :hash";
const INSERT_QUERY: &'static str = "INSERT INTO urls (url, hash, used_count) VALUES (:url, :hash, 0) ON CONFLICT DO NOTHING";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Url {
    id: u16,
    url: String,
    hash: String,
    used_count: u16
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    error: String,
}

pub async fn connect_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let connection = Connection::open("../data.sqlite")?;
    connection.execute(
        "CREATE TABLE IF NOT EXISTS urls (
             id INTEGER PRIMARY KEY,
             url TEXT NOT NULL,
             hash TEXT NOT NULL,
             used_count INTEGER NOT NULL
         )",
        (),
    )?;

    return Ok(connection);
}

pub async fn get_url(Path(hash): Path<String>) -> Result<Html<String>, (StatusCode, Json<ErrorResponse>)> {
    println!("hash: {:?}", &[(":hash", &hash)]);

    let connection = connect_db()
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to connect to database: {}", e),
            }))
        })?;
    println!("connection: {:?}", connection);

    let mut statement = connection
        .prepare(SELECT_QUERY)
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to prepare statement: {}", e),
            }))
        })?;
    println!("statement: {:?}", statement);

    let query_result = statement
        .query_row(&[(":hash", &hash)], |row| {
            Ok(row.get(0).expect("url not found"))
        })
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to execute query: {}", e),
            }))
        })?;

    return Ok(Html(query_result));
}

pub async fn create_hash(input_url: String) -> Result<Html<String>, (StatusCode, Json<ErrorResponse>)> {
    let connection = connect_db().await.map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            error: format!("Failed to connect to database: {}", e),
        }))
    })?;

    let mut hash = get_random_hash();
    loop {
        let mut select_statement = connection.prepare(SELECT_QUERY).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to prepare statement: {}", e),
            }))
        })?;

        let mut query_result = select_statement.query([&hash]).map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to execute query: {}", e),
            }))
        })?;

        if query_result.next().map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to fetch result: {}", e),
            }))
        })?.is_none() {
            break;
        }
        hash = get_random_hash();
    }

    let mut insert_statement = connection.prepare(INSERT_QUERY).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            error: format!("Failed to prepare statement: {}", e),
        }))
    })?;
    
    let _ = insert_statement.query(&[(":url", &input_url), (":hash", &hash)]).map_err(|e| {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
            error: format!("Failed to execute query: {}", e),
        }))
    })?;

    Ok(Html(hash))
}

fn get_random_hash() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const HASH_LENGTH: usize = 8;
    let mut rng = rand::thread_rng();

    let hash: String = (0..HASH_LENGTH)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    return hash;
}
