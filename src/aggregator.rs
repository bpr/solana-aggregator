mod utilities;

use futures_util::StreamExt;
use clap::{arg, Parser};
use nanodb::nanodb::NanoDB;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_response::SlotInfo;
use std::collections::VecDeque;
use tokio::task;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Displays debug logs from the application and dependencies
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// The number of slots lagged from first slot to first getBlock(slot).

    #[arg(long, default_value = "48")]
    lag: u32,

    /// The HTTP RPC URL for connecting to the Solana DevNet.
    #[arg(long, default_value = "")]
    rpc_url: String,

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
    let rpc_url = args.rpc_url.clone();
    let ws_url = args.ws_url.clone();
    let lag = args.lag as usize;

    let ps_client = PubsubClient::new(&ws_url).await?;
    let (mut accounts, unsubscriber) = ps_client.slot_subscribe().await?;
    let mut db = NanoDB::open(&args.output_file)?;

    let nblocks = match db.data().await.get("nblocks") {
        Ok(nblocks) => nblocks.into::<String>().unwrap_or("0".to_string()),
        Err(_) => "0".to_string(),
    };
    let mut count = nblocks.parse::<u32>().unwrap_or(0);
    let mut deque: VecDeque<SlotInfo> = VecDeque::with_capacity(lag);
    while let Some(response) = accounts.next().await {
        let rpc_url = rpc_url.clone();
        deque.push_back(response.clone());
        if deque.len() > lag {
            let slot_info = deque.pop_front().unwrap();
            let slot = slot_info.slot;
            let block = task::spawn_blocking(move || {
                // Call potentially costly synchronous code
                RpcClient::new(rpc_url).get_block(slot)
            }).await?;
            match block {
                Ok(block) => {
                    let key = format!("block-{}", slot);
                    if count == 0 {
                        db.insert("first_key", &key).await?;
                        db.insert("last_key", &key).await?;
                    } else {
                        db.insert("last_key", &key).await?;
                    }
                    count += 1;
                    // Insert the block into the database
                    db.insert(&key, &block).await?;
                    // Create a mapping of the block number to the key, so that we can
                    // retrieve the block key by number. Strictly speaking, this is not
                    // necessary, we could just use the number as index, but it is useful
                    // for keeping the slot <-> block mapping around
                    db.insert("nblocks", &format!("{count}")).await?;
                    db.insert(&format!("key_{count}"), &key).await?;
                    db.write().await?; // write to file if needed
                },
                Err(e) => println!("Error: {:?} at slot={slot}", e), // TODO: Log miss at slot#
            };
        };
        if args.max_blocks > 0 && count >= args.max_blocks {
            break;
        }
    }

    unsubscriber().await;
 
    Ok(())
}
