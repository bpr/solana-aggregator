use clap::{arg, Parser};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_client::{
    pubsub_client::PubsubClient,
    rpc_config::{RpcBlockSubscribeConfig, RpcBlockSubscribeFilter},
};
use solana_transaction_status::{TransactionDetails, UiTransactionEncoding};
use solana_sdk::commitment_config::CommitmentConfig;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Displays debug logs from the application and dependencies
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

   /// The WebSocket URL for connecting to the Solana DevNet.
    #[arg(long, default_value = "")]
    ws_url: String,

    /// The output file to write the blocks collected to, for NanoDB.
    #[arg(long, default_value = "solana_blocks.json")]
    output_file: String,

    /// The rate limit imposed on the cacher to prevent 429's on RPC.
    ///
    /// TODO: Implement rate limiting
    #[arg(long, default_value = "4")]
    rate_limit: u32,

    /// The maximum number of blocks to read. If 0, unlimited.
    #[arg(long, default_value = "16")]
    max_blocks: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let ws_url = args.ws_url.clone();

    // obtain full transactions for confirmed blocks
    let config = RpcBlockSubscribeConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        encoding: Some(UiTransactionEncoding::Base64),
        transaction_details: Some(TransactionDetails::Full),
        show_rewards: Some(false),
        max_supported_transaction_version: Some(0),
        ..Default::default()
    };
    // create the subscription with the defined configuration
    let (_tx_confirmed_block, rx_confirmed_block) =
        PubsubClient::block_subscribe(&ws_url,
                                      RpcBlockSubscribeFilter::All,
                                      Some(config)).unwrap();

    // loop through the subscription responses and print the block slot
    loop {
        match rx_confirmed_block.recv() {
            Ok(response) => {
                println!("{:?}", response.value.block);
            }
            Err(e) => {
                println!("Block Subscription Error: {:?}", e);
                break;
            }
        }
    }
    Ok(())
}
