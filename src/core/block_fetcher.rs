use clickhouse::Client;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcBlockConfig;
use solana_sdk::commitment_config::CommitmentConfig;
use solana_transaction_status::UiTransactionEncoding;
use super::block_analyzer::analyze_block;
use super::transaction_parser::parse_transactions;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct BlockRow {
    pub slot: u64,
    pub parent_slot: u64,
    pub blockhash: String,
    pub previous_blockhash: String,
    pub block_time: u64,
    pub transaction_count: u32,
}

pub async fn fetch_and_process_block(
    client: &Client,
    rpc_client: &RpcClient,
    slot: u64,
    alchemy_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Fetch block
    let block = rpc_client.get_block_with_config(
        slot,
        RpcBlockConfig {
            encoding: Some(UiTransactionEncoding::JsonParsed),
            transaction_details: Some(solana_transaction_status::TransactionDetails::Full),
            rewards: Some(true),
            commitment: Some(CommitmentConfig::finalized()),
            max_supported_transaction_version: Some(0),
        },
    )?;

    let block_time = block.block_time.unwrap_or(0) as u64;
    let tx_count = block.transactions.as_ref().map(|v| v.len()).unwrap_or(0) as u32;
    let blockhash = block.blockhash.clone();

    // Store block
    let block_row = BlockRow {
        slot,
        parent_slot: block.parent_slot,
        blockhash: blockhash.clone(),
        previous_blockhash: block.previous_blockhash.clone(),
        block_time,
        transaction_count: tx_count,
    };

    let mut inserter = client.insert("blocks")?;
    inserter.write(&block_row).await?;
    inserter.end().await?;

    // Parse transactions
    let transactions = if let Some(ref block_transactions) = block.transactions {
        parse_transactions(block_transactions, slot)
    } else {
        return Err("No transactions in block".into());
    };

    // Analyze block
    analyze_block(client, rpc_client, slot, blockhash, block_time, &transactions, alchemy_key).await?;

    Ok(())
}

pub fn should_skip_old_slot(err_str: &str, slot: u64, current_slot: u64) -> bool {
    if err_str.contains("cleaned up") || err_str.contains("does not exist") {
        if slot < current_slot.saturating_sub(100) {
            return true;
        }
    }
    false
}

