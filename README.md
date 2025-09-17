# Binance Net Flow Indexer #

This project tracks the cumulative net flow of POL tokens between the Polygon network and known Binance addresses.

It:

Listens to new Polygon blocks in real time

Stores raw transaction data in SQLite

Calculates net inflow/outflow of POL tokens into Binance

Prints live updates to the console

Features

Connects to Polygon RPC in real time

Detects transactions involving Binance addresses

Stores data in SQLite (polygon_data.db)

Tracks cumulative net flow of POL tokens

Provides live real-time updates

Setup & Installation

Clone the repository

git clone <your-repo-url>
cd <repo-name>


Install Rust
Follow the official Rust installation guide
.

Build dependencies

cargo build


Configure environment
Create a .env file in the root directory with the following:

POLYGON_RPC=https://polygon-rpc.com
POL_TOKEN=0x0000000000000000000000000000000000001010

Running the Indexer

Start real-time tracking:

cargo run --bin main


This will:

Connect to Polygon RPC

Listen for new blocks

Detect Binance-related transactions

Save data into SQLite

Print live net flow updates

Checking the Latest Net Flow

Run the query tool:

cargo run --bin total-flow


This prints the current cumulative net flow stored in the database.

Scalability & Future Improvements

Multi-exchange support: Add more exchange address lists

API Interface: Expose REST API endpoints instead of CLI

Dashboard: Web frontend with live charts and historical data

Resilience: Error handling, retry logic, and better logging

Tech Stack

Rust – Core language

SQLite – Data storage

Polygon RPC – Blockchain connection
