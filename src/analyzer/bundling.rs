use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::Transaction;
use super::landing_services::identify_landing_service;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct BundlingAnalysisRow {
    pub slot: u64,
    pub blockhash: String,
    pub block_time: u64,
    pub unique_blockhashes: u32,
    pub largest_blockhash_group: u32,
    pub largest_blockhash: String,
    pub validator_key: String,
    pub landing_service: String,  // Most common landing service in block (empty if unknown)
    pub landing_service_count: u32,  // Number of transactions using identified landing service
}

pub fn analyze_bundling(
    transactions: &[Transaction],
    slot: u64,
    blockhash: String,
    block_time: u64,
    validator_key: String,
) -> BundlingAnalysisRow {
    // Blockhash clustering
    let mut blockhash_counts: HashMap<String, u32> = HashMap::new();
    for tx in transactions {
        *blockhash_counts.entry(tx.recent_blockhash.clone()).or_insert(0) += 1;
    }
    
    let unique_blockhashes = blockhash_counts.len() as u32;
    let (largest_blockhash, largest_blockhash_group) = blockhash_counts
        .iter()
        .max_by_key(|(_, &count)| count)
        .map(|(bh, &count)| (bh.clone(), count))
        .unwrap_or((String::new(), 0));

    // Analyze tips from all transactions to identify landing services
    let mut tip_recipient_counts: HashMap<String, u32> = HashMap::new();
    let mut tip_amounts_by_recipient: HashMap<String, Vec<u64>> = HashMap::new();
    
    for tx in transactions {
        if !tx.tip_recipient.is_empty() && tx.tip_amount > 0 {
            *tip_recipient_counts.entry(tx.tip_recipient.clone()).or_insert(0) += 1;
            tip_amounts_by_recipient.entry(tx.tip_recipient.clone())
                .or_insert_with(Vec::new)
                .push(tx.tip_amount);
        }
    }
    
    // Identify landing service from tip patterns
    // If we see many transactions sending tips to the same address, that's likely a landing service
    let mut landing_service_found = String::new();
    let mut landing_service_count = 0u32;
    
    if let Some((recipient, count)) = tip_recipient_counts.iter().max_by_key(|(_, &c)| c) {
        // If many transactions tip the same address, that's the landing service
        if *count >= 2 {  // At least 2 transactions tipping same address
            // Try to identify the service
            if let Some(service) = identify_landing_service(recipient) {
                landing_service_found = service;
                landing_service_count = *count;
            } else {
                // Unknown landing service - use recipient address as identifier
                landing_service_found = format!("Unknown: {}", recipient);
                landing_service_count = *count;
            }
        }
    }

    BundlingAnalysisRow {
        slot,
        blockhash,
        block_time,
        unique_blockhashes,
        largest_blockhash_group,
        largest_blockhash,
        validator_key,
        landing_service: landing_service_found,
        landing_service_count,
    }
}

