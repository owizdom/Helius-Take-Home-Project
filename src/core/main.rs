#[path = "../analyzer/mod.rs"]
mod analyzer;

mod block_analyzer;
mod block_fetcher;
mod transaction_parser;

use clickhouse::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use block_fetcher::{fetch_and_process_block, should_skip_old_slot};

async fn init_clickhouse() -> Client {
    let password = std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| "solana123".to_string());
    Client::default()
        .with_url("http://localhost:8123")
        .with_database("solana")
        .with_user("default")
        .with_password(&password)
}

async fn initialize_database(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing database...");
    
    // Create database if it doesn't exist
    client.query("CREATE DATABASE IF NOT EXISTS solana").execute().await?;
    
    // Create blocks table
    client.query(
        "CREATE TABLE IF NOT EXISTS solana.blocks
        (
            slot UInt64,
            parent_slot UInt64,
            blockhash String,
            previous_blockhash String,
            block_time UInt64,
            transaction_count UInt32,
            created_at DateTime DEFAULT now()
        )
        ENGINE = MergeTree()
        ORDER BY (slot)
        PARTITION BY toYYYYMM(toDateTime(block_time))"
    ).execute().await?;
    println!("  Created table: blocks");
    
    // Create bundling_analysis table
    client.query(
        "CREATE TABLE IF NOT EXISTS solana.bundling_analysis
        (
            slot UInt64,
            blockhash String,
            block_time UInt64,
            unique_blockhashes UInt32,
            largest_blockhash_group UInt32,
            largest_blockhash String,
            validator_key String,
            landing_service String DEFAULT '',
            landing_service_count UInt32 DEFAULT 0,
            created_at DateTime DEFAULT now()
        )
        ENGINE = MergeTree()
        ORDER BY (slot)
        PARTITION BY toYYYYMM(toDateTime(block_time))"
    ).execute().await?;
    println!("  Created table: bundling_analysis");
    
    // Create fee_landscape table
    client.query(
        "CREATE TABLE IF NOT EXISTS solana.fee_landscape
        (
            slot UInt64,
            block_time UInt64,
            fee_avg Float64,
            compute_budget_percent Float32,
            fee_ordering_correlation Float32,
            created_at DateTime DEFAULT now()
        )
        ENGINE = MergeTree()
        ORDER BY (slot)
        PARTITION BY toYYYYMM(toDateTime(block_time))"
    ).execute().await?;
    println!("  Created table: fee_landscape");
    
    // Create program_fee_analysis table
    client.query(
        "CREATE TABLE IF NOT EXISTS solana.program_fee_analysis
        (
            slot UInt64,
            block_time UInt64,
            program_type String,
            program_name String,
            transaction_count UInt32,
            total_fee UInt64,
            min_fee UInt64,
            max_fee UInt64,
            created_at DateTime DEFAULT now()
        )
        ENGINE = MergeTree()
        ORDER BY (slot, program_type)
        PARTITION BY toYYYYMM(toDateTime(block_time))"
    ).execute().await?;
    println!("  Created table: program_fee_analysis");
    
    // Create fee_by_transaction_type table
    client.query(
        "CREATE TABLE IF NOT EXISTS solana.fee_by_transaction_type
        (
            slot UInt64,
            block_time UInt64,
            transaction_type String,
            transaction_count UInt32,
            total_fee UInt64,
            created_at DateTime DEFAULT now()
        )
        ENGINE = MergeTree()
        ORDER BY (slot, transaction_type)
        PARTITION BY toYYYYMM(toDateTime(block_time))"
    ).execute().await?;
    println!("  Created table: fee_by_transaction_type");
    
    println!("Database initialized.\n");
    Ok(())
}

async fn clear_database(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    println!("Clearing database...");
    let tables = vec![
        "blocks",
        "bundling_analysis",
        "program_fee_analysis",
        "fee_landscape",
        "fee_by_transaction_type",
    ];
    
    for table in tables {
        match client.query(&format!("TRUNCATE TABLE IF EXISTS {}", table)).execute().await {
            Ok(_) => println!("  Cleared table: {}", table),
            Err(e) => eprintln!("  Warning: Failed to clear table {}: {:?}", table, e),
        }
    }
    
    // Ensure bundling_analysis table has the correct columns
    let alter_queries = vec![
        "ALTER TABLE bundling_analysis ADD COLUMN IF NOT EXISTS landing_service String DEFAULT ''",
        "ALTER TABLE bundling_analysis ADD COLUMN IF NOT EXISTS landing_service_count UInt32 DEFAULT 0",
        "ALTER TABLE bundling_analysis DROP COLUMN IF EXISTS jito_bundle_count",
        "ALTER TABLE bundling_analysis DROP COLUMN IF EXISTS direct_leader_count",
    ];
    
    for query in alter_queries {
        let _ = client.query(query).execute().await;
    }
    
    println!("Database cleared.\n");
    Ok(())
}

async fn get_last_processed_slot(client: &Client) -> u64 {
    client
        .query("SELECT slot FROM blocks ORDER BY slot DESC LIMIT 1")
        .fetch_one()
        .await
        .unwrap_or(0)
}

async fn initialize_from_current_slot(rpc_client: &RpcClient) -> u64 {
    match rpc_client.get_slot() {
        Ok(current_slot) => {
            let start_slot = current_slot.saturating_sub(5);
            println!("Database empty, starting from slot {}", start_slot);
            start_slot
        }
        Err(e) => {
            eprintln!("WARNING: Failed to get current slot: {:?}", e);
            eprintln!("Will retry in 5 seconds...");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            0
        }
    }
}

async fn get_current_slot(rpc_client: &RpcClient) -> Result<u64, Box<dyn std::error::Error>> {
    rpc_client.get_slot().map_err(|e| e.into())
}

async fn process_slots(
    client: &Client,
    rpc_client: &RpcClient,
    start_slot: u64,
    current_slot: u64,
    last_processed_slot: &mut u64,
    alchemy_key: &str,
) {
    let end_slot = current_slot.min(start_slot + 10);

    for slot in start_slot..=end_slot {
        match fetch_and_process_block(client, rpc_client, slot, alchemy_key).await {
            Ok(_) => {
                println!("Block {} fetched and analyzed", slot);
                *last_processed_slot = slot;
            }
            Err(e) => {
                let err_str = format!("{:?}", e);
                if should_skip_old_slot(&err_str, slot, current_slot) {
                    println!("Skipping old slot {} (not available), jumping to recent slots", slot);
                    *last_processed_slot = current_slot.saturating_sub(10);
                    break;
                } else if slot == current_slot {
                    break;
                } else {
                    eprintln!("WARNING: Failed to process block {}: {}", slot, e);
                }
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("\n=== Welcome to Solana Block Analyzer ===\n");
    
    // Hardcoded API keys
    let getblock_token = "1f95ca0bb31f442f8a598bd1d4043da9";
    let alchemy_key = "AFjoSzKjqv6Eq53OsF2xe";
    
    // Display ClickHouse credentials
    let clickhouse_password = std::env::var("CLICKHOUSE_PASSWORD").unwrap_or_else(|_| "solana123".to_string());
    println!("\n=== ClickHouse Configuration ===");
    println!("Username: default");
    println!("Password: {}", clickhouse_password);
    println!("Database: solana");
    println!("Dashboard: http://localhost:8123/play");
    println!("===============================\n");
    
    let client = init_clickhouse().await;
    
    // Initialize database (create tables if they don't exist)
    if let Err(e) = initialize_database(&client).await {
        eprintln!("Error: Failed to initialize database: {:?}", e);
        eprintln!("Please ensure ClickHouse is running and accessible.");
        return;
    }
    
    // Clear database on each run
    if let Err(e) = clear_database(&client).await {
        eprintln!("Warning: Failed to clear database: {:?}", e);
        eprintln!("Continuing anyway...\n");
    }
    
    // Initialize RPC client with user's GetBlock token
    let rpc_url = format!("https://go.getblock.us/{}", getblock_token);
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::finalized());

    println!("Starting Solana block fetcher & analyzer - streaming mode");
    println!("Monitoring for new blocks...\n");

    let mut last_processed_slot = get_last_processed_slot(&client).await;

    if last_processed_slot == 0 {
        last_processed_slot = initialize_from_current_slot(&rpc_client).await;
        if last_processed_slot == 0 {
            return;
        }
    }

    loop {
        match get_current_slot(&rpc_client).await {
            Ok(current_slot) => {
                if current_slot > last_processed_slot {
                    process_slots(&client, &rpc_client, last_processed_slot + 1, current_slot, &mut last_processed_slot, &alchemy_key).await;
                } else {
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
            Err(e) => {
                eprintln!("WARNING: Failed to get slot: {:?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
            }
        }
    }
}
