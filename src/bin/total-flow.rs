use rusqlite::Connection;

fn main() {
    let conn = Connection::open("polygon_data.db").unwrap();

    let mut stmt = conn.prepare(
        "SELECT block_number, deposit, withdrawal, net_flow
         FROM net_flows
         ORDER BY id DESC LIMIT 1"
    ).unwrap();

    let row = stmt.query_row([], |row| {
        Ok((
            row.get::<_, i64>(0)?, 
            row.get::<_, String>(1)?, 
            row.get::<_, String>(2)?, 
            row.get::<_, String>(3)?, 
        ))
    }).unwrap();

    let (block, deposit, withdrawal, net_flow) = row;

    let dep_f = deposit.parse::<f64>().unwrap() / 1e18;
    let wit_f = withdrawal.parse::<f64>().unwrap() / 1e18;
    let net_f = net_flow.parse::<f64>().unwrap() / 1e18;

    println!("📊 Latest Net Flow at Block {}:", block);
    println!("   Deposits   : {:.6} POL", dep_f);
    println!("   Withdrawals: {:.6} POL", wit_f);
    println!("   Net Flow   : {:.6} POL", net_f);
}
