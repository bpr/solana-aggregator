# Solana Aggregator
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Overview
Solana Aggregator is a suite of tools built in Rust for the Solforge interview. It is designed to efficiently pull blocks from the Solana blockchain, respecting the specified rate limits. 

## Requirements
- **Data Retrieval:** Must be able to retrieve transaction and account data from the Solana devnet or testnet, using Solana's API or SDK.
- **Data Processing:** Implement mechanisms to process the retrieved data efficiently. This includes parsing transaction records, extracting relevant information such as sender, receiver, amount, timestamp, etc., and organising data into a structured format for further analysis and queries.
- **Data History:** Configure the data aggregator to start aggregating data from the current epoch and onwards. Exclude historical data to focus on recent transactions and account changes. Ensure the data aggregator provides real-time updates by continuously monitoring the blockchain for new transactions and account changes.
- **Data Storage (optional):** Choose a storage solution to store the collected data securely. Consider using a suitable database or data storage mechanism that offers scalability, reliability, and fast query capabilities. If you are running out of time, a in-memory structure is enough!
- **API Integration:** Create a RESTful API layer to expose the aggregated data to external systems and applications. The API should support various queries to retrieve transaction history, account details, and other relevant information.

## Design
Initial suggestions to use a WebSocket API to satisfy the **Data History** requirement for continuous monitoring were stymied by the fact that the `blockSubscribe` WS method 
is unstable, and that the `slotSubscribe` WS method returned slots that were not readable
via the `getBlock` method. After experimentation with the `getBlocks` method, I found that if I introduced a lag of about 36 slots, I could use the slots from `slotScubscribe` with the `getBlock` method. Another alternative that I started working on was to forego the WS API entirely, and use a combination of the `getEpochInfo` RPC method, and `getBlocks` to fetch ranges of available blocks; this would have been a hand written polling approach. The lagged slots approach described above is much simpler.

Having a means to get a continuous feed of recent blocks, I decided that I would store all of those blocks "as is" to be read by the RESTful API. After examining  a few Rust based alternatives, I decided to use [NanoDB](https://crates.io/crates/nanodb) which provides a simple layer for accessing JSON data, which I obviously have access to given Serde. An attacker who has access to the machines upon which the data is stored will be able to easily see the data, as the DB files are human readable JSON. I decided to ignore this problem in the interests of time.

## Installation

```bash
cargo install solana-block-cacher
```

## Usage
To use Solana Block Cacher, run the command with the desired arguments. Below are the available options:
```bash
solana-block-cacher [OPTIONS]
```

### Getting an RPC URL
You can get a Solana RPC Url through [QuickNode](https://www.quicknode.com?tap_a=67226-09396e&tap_s=4369813-07359f&utm_source=affiliate&utm_campaign=generic&utm_content=affiliate_landing_page&utm_medium=generic). 
I personally use the QuickNode Pro solution which allows me to retrieve millions of blocks a month for back testing.

### Options
- `-v, --verbose`: Enables verbose logging (default: false).
- `-o, --output_file <OUTPUT_FILE>`: The file to write the blocks to (default: "blocks.json").
- `--rate_limit <RATE_LIMIT>`: The rate limit for the cacher (default: 50 (QuickNode Pro Default)).
- `-w, --window <WINDOW>`: The time window for the rate limit in seconds (default: 1 (QuickNode Pro Default)).

### Example

## License
This project is open source and available under MIT.

#### Happy Coding! 🚀🦀

