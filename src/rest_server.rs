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
use clap::{arg, Parser};
use nanodb::{error::NanoDBError, nanodb::NanoDB};
use std::collections::HashMap;
use tokio::net::TcpListener;

pub struct MyNanoDBError(NanoDBError);

impl From<NanoDBError> for MyNanoDBError {
    fn from(error: NanoDBError) -> Self {
        MyNanoDBError(error)
    }
}

impl IntoResponse for MyNanoDBError {
        fn into_response(self) -> Response {
        let body = "something went wrong";
/*
        use NanoDBError::*;
        match self.0 {
            DeserializeFromStr(Error) => "Failed to deserialize from string",
            Io(Error) => "I/O error",
            RwLockReadError(String) => "Read error on RwLock",
            RwLockWriteError(String),
            NotAnArray(String),
            LenNotDefined(String),
            NotAnObject(String),
            KeyNotFound(String),
            IndexOutOfBounds(usize),
            InvalidJSONPath,
            TypeMismatch(String),
            DefaultError,
    } 
 */
        // its often easiest to implement `IntoResponse` by calling other implementations
        body.into_response()
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Displays debug logs from the application and dependencies
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The HTTP RPC URL for connecting to the Solana DevNet.
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
    let rpc_url = args.rpc_url.clone();
    let server_address = args.server_address.clone();
    let mut db = NanoDB::open(&args.db_file)?;

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

#[debug_handler]
async fn get_nblocks(
    State(db): State<NanoDB>,
) -> Result<String, MyNanoDBError> {
    let number = db.data().await.get("nblocks")?.into::<u64>()?;
    Ok(number.to_string())
}

#[debug_handler]
async fn get_block(
    State(db): State<NanoDB>,
    Path(index): Path<u64>,
) -> Result<String, MyNanoDBError> {
    let key = db.data().await.get(&format!("key_{}", index))?.into::<String>()?;
    let block = db.data().await.get(&key)?.into::<u64>()?;
    Ok(block.to_string())
}

#[debug_handler]
async fn get_transactions(
    State(db): State<NanoDB>,
    Query(params): Query<HashMap<String, String>>
) -> Result<String, MyNanoDBError> {
    let mut transactions: Vec<String> = Vec::new();
    for (key, value) in &params {
        if key == "id" {
            ();
        } else if key == "day" {
            ();
        } else {
            return Err(NanoDBError::KeyNotFound(key.to_string()).into());
        }
    }
    Ok("".to_string())
}

#[debug_handler]
async fn get_account_by_key(
        State(_db): State<NanoDB>,
    ) -> Result<String, MyNanoDBError> {
    todo!()
}
