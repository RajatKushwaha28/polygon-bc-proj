use std::env;
use web3::transports::Http;
use web3::Web3;
use web3::types::{BlockId, BlockNumber, U256};
use rusqlite::Connection;
use dotenv::dotenv;
use tokio::time::{sleep, Duration};

const BINANCE_ADDRESSES: [&str; 6] = [
    "0xF977814e90dA44bFA03b6295A0616a897441aceC",
    "0xe7804c37c13166fF0b37F5aE0BB07A3aEbb6e245",
    "0x505e71695E9bc45943c58adEC1650577BcA68fD9",
    "0x290275e3db66394C52272398959845170E4DCb88",
    "0xD5C08681719445A5Fdce2Bda98b341A49050d821",
    "0x082489A616aB4D46d1947eE3F912e080815b08DA",
];

fn is_binance(addr: &str) -> bool {
    BINANCE_ADDRESSES.iter().any(|&a| a.eq_ignore_ascii_case(addr))
}

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok();

    let rpc_url = env::var("POLYGON_RPC").expect("RPC URL not set");
    let transport = Http::new(&rpc_url)?;
    let web3 = Web3::new(transport);

    println!("✅ Connected to Polygon RPC: {}", rpc_url);

    let conn = Connection::open("polygon_data.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            block_number INTEGER,
            from_addr TEXT,
            to_addr TEXT,
            value TEXT
        )",
        [],
    ).unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS net_flows (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            block_number INTEGER,
            deposit TEXT,
            withdrawal TEXT,
            net_flow TEXT
        )",
        [],
    ).unwrap();

    let mut last_block = web3.eth().block_number().await?.as_u64();
    let mut cumulative_deposit: U256 = U256::zero();
    let mut cumulative_withdrawal: U256 = U256::zero();

    loop {
        let latest_block = web3.eth().block_number().await?.as_u64();

        if latest_block > last_block {
            for block_num in (last_block + 1)..=latest_block {
                if let Ok(Some(block)) = web3.eth()
                    .block_with_txs(BlockId::Number(BlockNumber::Number(block_num.into())))
                    .await
                {
                    for tx in block.transactions {
                        let from = format!("{:?}", tx.from);
                        let to = tx.to.map(|t| format!("{:?}", t)).unwrap_or("None".to_string());
                        let value = tx.value;
                        conn.execute(
                            "INSERT INTO transactions (block_number, from_addr, to_addr, value) VALUES (?1, ?2, ?3, ?4)",
                            (block_num as i64, from.clone(), to.clone(), value.to_string()),
                        ).unwrap();
                        if value > U256::zero() {
                            if is_binance(&to) {
                                cumulative_deposit += value;
                            }
                            if is_binance(&from) {
                                cumulative_withdrawal += value;
                            }
                        }
                    }

                    let net = cumulative_deposit.saturating_sub(cumulative_withdrawal);

                    conn.execute(
                        "INSERT INTO net_flows (block_number, deposit, withdrawal, net_flow) VALUES (?1, ?2, ?3, ?4)",
                        (block_num as i64,
                         cumulative_deposit.to_string(),
                         cumulative_withdrawal.to_string(),
                         net.to_string()),
                    ).unwrap();

                    let net_in_matic = net.as_u128() as f64 / 1e18;
                    println!("📦 Block {} → NetFlow = {:.6} POL", block_num, net_in_matic);

                }
            }

            last_block = latest_block;
        }

        sleep(Duration::from_secs(5)).await;
    }
}
