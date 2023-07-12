use axum::{
    extract::Path,
    http::StatusCode,
    response::{Html, Json, Redirect}
};
use rusqlite::{Connection, OptionalExtension};
use serde::{Serialize, Deserialize};

const URL: &'static str = "http://localhost:3000";
pub const SELECT_URL_QUERY: &'static str = "SELECT url FROM urls WHERE hash = :hash";
const SELECT_HASH_QUERY: &'static str = "SELECT hash FROM urls WHERE url = :url";
const INSERT_QUERY: &'static str = "INSERT INTO urls (url, hash, used_count) VALUES (:url, :hash, 0)";

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

#[derive(serde::Deserialize)]
pub struct CreateHashRequestBody {
    input_url: String,
}

pub async fn connect_db() -> Result<Connection, Box<dyn std::error::Error>> {
    let connection = Connection::open("./data.sqlite")?;
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

pub async fn redirect(Path(hash): Path<String>) -> Result<Redirect, (StatusCode, Json<ErrorResponse>)> {
    let connection = connect_db()
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to connect to database: {}", e),
            }))
        })?;

    let query_result: String = connection
        .query_row(SELECT_URL_QUERY, &[(":hash", &hash)], |row| {
            Ok(row.get(0).expect("url not found"))
        })
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to execute query: {}", e),
            }))
        })?;

    return Ok(Redirect::to(&query_result.to_string()));
}

pub async fn create_hash(Json(body): Json<CreateHashRequestBody>) -> Result<Html<String>, (StatusCode, Json<ErrorResponse>)> {
    let input_url = body.input_url;

    let connection = connect_db()
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to connect to database: {}", e),
            }))
        })?;

    let query_result: Option<String> = connection
        .query_row(SELECT_HASH_QUERY, &[(":url", &input_url)], |row| {
            Ok(row.get(0).expect("url not found"))
        })
        .optional()
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                error: format!("Failed to execute query: {}", e),
            }))
        })?;

    match query_result {
        Some(query_result) => {
            return Ok(Html(format!(
                r#"
                    <div class="flex flex-col gap-4">
                        <h2>Here's your shortened url</h2>
                        <div class='w-full px-6 py-2 rounded-lg shadow bg-white'>{}/{}</div>
                    </div>
                "#, URL, query_result
            )));
        }
        None => {
            let mut hash = get_random_hash();
            loop {
                let mut select_statement = connection
                    .prepare(SELECT_URL_QUERY)
                    .map_err(|e| {
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                            error: format!("Failed to prepare statement: {}", e),
                        }))
                    })?;

                let hash_exists = select_statement
                    .exists(&[(":hash", &hash)])
                    .map_err(|e| {
                        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                            error: format!("Failed to execute query: {}", e),
                        }))
                    })?;

                if !hash_exists { break; }
                hash = get_random_hash();
            }
            
            connection
                .execute(INSERT_QUERY, &[(":url", &input_url), (":hash", &hash)])
                .map_err(|e| {
                    (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse {
                        error: format!("Failed to execute query: {}", e),
                    }))
                })?;

            let html = format!("<div class='w-full px-6 py-2 rounded-lg shadow bg-white'>{}/{}</div>", URL, hash);

            return Ok(Html(html));
        }
    }
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
