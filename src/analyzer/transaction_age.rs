use clickhouse::Client;
use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use super::types::Transaction;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct TransactionAgeRow {
    pub slot: u64,
    pub block_time: u64,
    pub validator_key: String,
    pub max_transaction_age_slots: u64,
    pub avg_transaction_age_slots: f64,
    pub old_transaction_count: u32,  // Transactions >150 slots old
    pub total_transactions: u32,
}

/// Lookup blockhash creation slot from blocks table
async fn get_blockhash_slot(
    client: &Client,
    blockhash: &str,
) -> Result<Option<u64>, Box<dyn std::error::Error>> {
    // Query blocks table to find when this blockhash was created
    let query = format!(
        "SELECT slot FROM blocks WHERE blockhash = '{}' ORDER BY slot ASC LIMIT 1",
        blockhash
    );
    
    match client.query(&query).fetch_one::<u64>().await {
        Ok(slot) => Ok(Some(slot)),
        Err(clickhouse::error::Error::RowNotFound) => Ok(None),
        Err(e) => {
            // If query fails, return None (blockhash not found in our blocks table)
            eprintln!("Warning: Failed to query blockhash {}: {:?}", blockhash, e);
            Ok(None)
        }
    }
}

pub async fn analyze_transaction_age(
    client: &Client,
    transactions: &[Transaction],
    slot: u64,
    block_time: u64,
    validator_key: String,
) -> Result<TransactionAgeRow, Box<dyn std::error::Error>> {
    // Collect unique blockhashes
    let unique_blockhashes: HashSet<String> = transactions
        .iter()
        .map(|tx| tx.recent_blockhash.clone())
        .filter(|bh| !bh.is_empty())
        .collect();

    // Lookup blockhash creation slots
    let mut blockhash_to_slot: HashMap<String, Option<u64>> = HashMap::new();
    for blockhash in &unique_blockhashes {
        let creation_slot = get_blockhash_slot(client, blockhash).await?;
        blockhash_to_slot.insert(blockhash.clone(), creation_slot);
    }

    // Calculate transaction ages
    let mut transaction_ages: Vec<u64> = Vec::new();
    let mut old_transaction_count = 0u32;
    const OLD_TRANSACTION_THRESHOLD: u64 = 150; // Transactions older than 150 slots are considered "old"

    for tx in transactions {
        if tx.recent_blockhash.is_empty() {
            continue;
        }

        if let Some(Some(blockhash_slot)) = blockhash_to_slot.get(&tx.recent_blockhash) {
            let age = slot.saturating_sub(*blockhash_slot);
            transaction_ages.push(age);
            
            if age > OLD_TRANSACTION_THRESHOLD {
                old_transaction_count += 1;
            }
        }
    }

    let total_transactions = transactions.len() as u32;
    
    // Calculate statistics
    let max_transaction_age_slots = transaction_ages.iter().max().copied().unwrap_or(0);
    let avg_transaction_age_slots = if !transaction_ages.is_empty() {
        transaction_ages.iter().sum::<u64>() as f64 / transaction_ages.len() as f64
    } else {
        0.0
    };

    Ok(TransactionAgeRow {
        slot,
        block_time,
        validator_key,
        max_transaction_age_slots,
        avg_transaction_age_slots,
        old_transaction_count,
        total_transactions,
    })
}

