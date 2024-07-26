mod utilities;

// use solana_program::pubkey::Pubkey;
use axum::{
    debug_handler,
    extract::{Path, Query, State},
    // http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    // Json,
    Router,
};
use chrono::DateTime;
use clap::{arg, Parser};
use nanodb::{error::NanoDBError, nanodb::NanoDB};
use solana_transaction_status::EncodedConfirmedBlock;
use std::collections::HashMap;
use tokio::net::TcpListener;
use utilities::txn_utils::contains_signature;

const SEC_PER_DAY: i64 = 86400;

pub struct MyNanoDBError(NanoDBError);

impl From<NanoDBError> for MyNanoDBError {
    fn from(error: NanoDBError) -> Self {
        MyNanoDBError(error)
    }
}

impl IntoResponse for MyNanoDBError {
    fn into_response(self) -> Response {
        use NanoDBError::*;
        let body = match self.0 {
            DeserializeFromStr(_e) => format!("Failed to deserialize from string"),
            Io(_e) => format!("I/O error"),
            RwLockReadError(_s) => format!("Read error on RwLock"),
            RwLockWriteError(_s) => format!("Write error on RwLock"),
            NotAnArray(_s) => format!("Not an array"),
            LenNotDefined(_s) => format!("Length not defined"),
            NotAnObject(_s) => format!("Not an object"),
            KeyNotFound(_s) => format!("Key not found"),
            IndexOutOfBounds(_u) => format!("Index out of bounds"),
            InvalidJSONPath => format!("Invalid JSON path"),
            TypeMismatch(s) => format!("Type mismatch: {}", s),
            DefaultError => format!("Default error"),
        };
        body.into_response()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Displays debug logs from the application and dependencies
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The REST API endpoint for this server.
    #[arg(long, default_value = "127.0.0.1:3000")]
    server_address: String,

    /// The HTTP RPC URL for connecting to the Solana DevNet.
    #[arg(long, default_value = "")]
    rpc_url: String,

    /// The output file to read the blocks collected from the blockchain
    #[arg(long, default_value = "solana_blocks.json")]
    db_file: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let _rpc_url = args.rpc_url.clone();
    let server_address = args.server_address.clone();
    let db = NanoDB::open(&args.db_file)?;

    // Create the TCP listener
    let listener = TcpListener::bind(&server_address)
        .await
        .expect("Failed to bind to the server address");
    println!("Listening on {}", listener.local_addr().unwrap());

    // Compose the routes
    let app = Router::new()
        .route("/nblocks", get(get_nblocks))
        .route("/block/:index", get(get_block))
        .route("/transactions/", get(get_transactions))
        .route("/account/:pubkey", get(get_account_by_key))
        .with_state(db);
    // Serve the application

    axum::serve(listener, app).await.expect("Error running the server");
    Ok(())
}

async fn get_nblocks(
    State(db): State<NanoDB>,
) -> Result<String, MyNanoDBError> {
    let number = db.data().await.get("nblocks")?.into::<String>()?;
    Ok(number)
}

async fn get_block(
    State(db): State<NanoDB>,
    Path(index): Path<u64>,
) -> Result<String, MyNanoDBError> {
    let key = db.data().await.get(&format!("key_{}", index))?.into::<String>()?;
    let block: EncodedConfirmedBlock = db.data().await.get(&key)?.into()?;
    Ok(serde_json::to_string(&block).unwrap())
}

async fn get_transactions(
    State(db): State<NanoDB>,
    Query(params): Query<HashMap<String, String>>
) -> Result<String, MyNanoDBError> {
    let mut transactions: Vec<String> = Vec::new();
    for (key, value) in &params {
        if key == "id" {
            let nblocks_string = db.data().await.get("nblocks")?.into::<String>()?;
            let nblocks = nblocks_string.parse::<u64>().unwrap();
            for i in 1..=nblocks {
                let key = db.data().await.get(&format!("key_{}", i))?.into::<String>()?;
                let block = db.data().await.get(&key)?.into::<EncodedConfirmedBlock>()?;
                for tx in &block.transactions {
                    if contains_signature(&tx, &value) {
                        transactions.push(serde_json::to_string(&tx).unwrap());
                    }
                }
            }
        } else if key == "day" {
            let date_string = format!("{value} 00:00:00 +0000");
            let dt = DateTime::parse_from_str(&date_string, "%d/%m/%Y %H:%M:%S %z").unwrap();
            let lo = dt.timestamp();
            let hi = lo + SEC_PER_DAY;
            let nblocks_string = db.data().await.get("nblocks")?.into::<String>()?;
            let nblocks = nblocks_string.parse::<u64>().unwrap();
            for i in 1..=nblocks {
                let key = db.data().await.get(&format!("key_{}", i))?.into::<String>()?;
                let block = db.data().await.get(&key)?.into::<EncodedConfirmedBlock>()?;
                if let Some(block_time) = block.block_time {
                    if block_time >= lo && block_time < hi {
                        for tx in &block.transactions {
                            transactions.push(serde_json::to_string(&tx).unwrap());
                        }
                    }
                }
            }
        } else {
            return Err(NanoDBError::KeyNotFound(key.to_string()).into());
        }
    }
    Ok(transactions.join("\n"))
}

#[debug_handler]
async fn get_account_by_key(
        State(_db): State<NanoDB>,
    ) -> Result<String, MyNanoDBError> {
    todo!()
}
