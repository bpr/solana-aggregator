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

## Evaluation Criteria
**Evaluation Criteria:**

- **Functionality:** Does the data aggregator retrieve and process Solana blockchain data accurately and efficiently?

The aggregator is inaccurate in that I introduced a **lag** so that I could use the `slotSubscibe` WebSocket interface. I was able to get the unstable `blockSubscribe` to work on QuickNode instances, but it didn't work with Solana or Helius devnets, so I abandoned that approach. Another approach would have been to just use the RPC calls, as I described in an email. It might be worth reconsidering for a production version.

- **Performance:** How well does the application handle large volumes of data and concurrent requests?

The application is probably much slower than it needs to be. If I were designing it for speed, I'd have the REST API entirely specified, and design the application to make those calls fast. For example, if I expected a lot of calls to get transactions or blocks by date, I'd date shard the database. I'd also consider storing the database in a RAM cache, like Redis. If we're interested in recent data, that suggests we could expire data to keep the cache from becoming enormous.

- **Reliability:** Is the data aggregator resilient to failures and capable of recovering gracefully?

There are still `unwrap` calls in the code that I haven't removed that are potential sources of runtime panics. I'll remove them when I have the time.

- **Scalability:** Can the application scale to handle increasing data loads without sacrificing performance?

As mentioned in the **Performance** section, I'd do a few things differently for performance.

- **Security:** Are proper security measures implemented to protect data integrity?

No. I assume right now that I'm running in a secure environment. I didn't, for example, encrypt the database. 

- **Documentation and Maintainability:** Is the codebase well-documented, well-composed, maintainable, and easy to understand for future developers?

The codebase should be very easy to understand. Each piece is simple, one piece simply gets data from Solana, the other is a REST API that only uses GET methods.

## Installation

```bash
cargo install solana-block-cacher
```

## Usage
To run the aggregator, run the command with the desired arguments. Below are the available options:
```bash
cargo run --bin aggregator -- --rpc-url https://api.devnet.solana.com --ws-url wss://api.devnet.solana.com
```

To run the rest server, run the command with the desired arguments. Below are the available options:
```bash
cargo run --bin rest_server -- --rpc-url https://api.devnet.solana.com
```

Remember to run the rest server *after* the aggregator is running, and to ensure that the DB file of the rest_server is the same as the output file of the aggregator.

To call the rest server, here are some examples with the current API.

```
curl 'http://localhost:3000/nblocks'
curl 'http://localhost:3000/block/12'
curl 'http://localhost:3000/transactions/?day=25/07/2024'
curl 'http://localhost:3000/transactions/?id=3giGDPLTP6mHDMDc21SPd71FnSKc5KRZKtx58tS95yMUoTVJeRKzzDrG2eE6ZhAjWaVEan7TjnYurntPVq53kkjR'
```

### Options

For the aggregator
Usage: aggregator [OPTIONS]

Options:
-  `-v, --verbose`
          Displays debug logs from the application and dependencies

-      `--lag <LAG>`
          The number of slots lagged from first slot to first getBlock(slot)
                    
          [default: 48]

-      `--rpc-url <RPC_URL>`
          The HTTP RPC URL for connecting to the Solana DevNet
          
          [default: ]

-      `--ws-url <WS_URL>`
          The WebSocket URL for connecting to the Solana DevNet.
          
          [default: ]

-      `--output-file <OUTPUT_FILE>`
          The output file to write the blocks collected to, for NanoDB.
          
          [default: solana_blocks.json]

-      `--rate-limit <RATE_LIMIT>`
          The rate limit imposed on the cacher to prevent 429's on RPC.
          
          TODO: Implement rate limiting
          
          [default: 4]
-      `--max_blocks <MAX_BLOCKS>`
          The maximum number of blocks to read. If 0, unlimited.
    
          [default: 16]

-  `-h, --help`
          Print help (see a summary with '-h')

-  `-V, --version`
          Print version

For the rest_server
Usage: rest_server [OPTIONS]

Options:
-  -v, --verbose
          Displays debug logs from the application and dependencies
      --server-address <SERVER_ADDRESS>
          The REST API endpoint for this server [default: 127.0.0.1:3000]
      --rpc-url <RPC_URL>
          The HTTP RPC URL for connecting to the Solana DevNet [default: ]
      --db-file <DB_FILE>
          The output file to read the blocks collected from the blockchain [default: solana_blocks.json]
  -h, --help
          Print help
  -V, --version
          Print version

### Example

## License
This project is open source and available under MIT.

#### Happy Coding! 🚀🦀

