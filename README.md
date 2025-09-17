Binance Net Flow Indexer

This project tracks the cumulative net flow of POL tokens between the Polygon network and known Binance addresses.
It listens to new blocks in real time, stores transaction data, and calculates how much net POL has moved into and out of Binance.

Setup & Installation
1. Clone the repo
2. Install Rust
3. Install Dependencies
    cargo build
4. Set Polygon RPC URL and POL_TOKEN in .env

Running the Indexer

Start the real-time indexer:
cargo run --bin main
This will:

Connect to the Polygon RPC

Listen for new blocks

Detect transactions involving Binance addresses

Save raw transactions + net flow in SQLite (polygon_data.db)

Print net flow updates in real-time

Checking the Latest Net Flow

Run the query tool:

cargo run --bin total-flow


Scalability & Future Improvements

Multi-exchange support: Add more exchange address lists (e.g., Coinbase, Kraken).

API Interface: Expose a simple REST API endpoint instead of CLI.

Dashboard: Add a web frontend for live charts and historical views.

Resilience: Add error handling, retry logic, and logging improvements.