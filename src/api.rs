use rusqlite::{Connection, params};
use leptos::{server, ServerFnError};
use serde::{Serialize, Deserialize};

const SELECT_QUERY: &'static str = "SELECT * FROM urls WHERE hash = ?";
const INSERT_QUERY: &'static str = "INSERT INTO users (url, hash, used_count) VALUES (?, ?, 0)";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize,)]
struct Url {
    id: u16,
    url: String,
    hash: String,
    used_count: u16
}

pub async fn connect_db() -> Result<Connection, ServerFnError> {
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

#[server(GetUrl, "/api", "GetJson")]
pub async fn get_url(hash: String) -> Result<String, ServerFnError> {
    let connection = connect_db().await?;
    
    let mut statement = connection.prepare(SELECT_QUERY)?;
    let mut query_result = statement.query(params![hash])?;
    let first_row = query_result.next()?;
    let url: String = first_row.expect("url not found").get(1)?;

    return Ok(url);
}

#[server(GenerateUrl, "/api")]
pub async fn generate_url(input_url: String) -> Result<String, ServerFnError> {
    let connection = connect_db().await?;

    let mut hash = get_random_hash();
    loop {
        let mut select_statement = connection.prepare(SELECT_QUERY)?;
        let mut query_result = select_statement.query(params![&hash])?;

        if query_result.next()?.is_none() {
            break;
        }

        hash = get_random_hash();
    }

    let mut insert_statement = connection.prepare(INSERT_QUERY)?;
    let mut query_result = insert_statement.query(params![input_url, hash])?;
    let first_row = query_result.next()?;
    let url: String = first_row.expect("url not found").get(1)?;

    return Ok(url);
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
