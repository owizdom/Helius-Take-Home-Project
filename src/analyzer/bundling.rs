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
    pub largest_bundle_size: u32,  // Size of largest detected bundle (up to 5 transactions)
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
    // Detect Jito bundles: sequential transactions (up to 5) with same tip recipient
    // According to Jito docs: bundles are groups of transactions (max 5) executed sequentially and atomically
    // We can only reliably detect bundles by identifying transactions with tips to the same recipient
    // Note: We cannot identify non-tip transactions as part of bundles without bundle IDs
    let mut bundle_clusters: Vec<Vec<usize>> = Vec::new();
    let mut current_bundle: Vec<usize> = Vec::new();
    let mut current_tip_recipient: Option<String> = None;
    
    for (idx, tx) in transactions.iter().enumerate() {
        if !tx.tip_recipient.is_empty() && tx.tip_amount > 0 {
            // Transaction has a tip - this is part of a bundle
            // Check if this transaction continues the current bundle (same tip recipient, sequential)
            if let Some(ref tip_recip) = current_tip_recipient {
                if tx.tip_recipient == *tip_recip && current_bundle.len() < 5 {
                    // Continue current bundle (same tip recipient, sequential, under limit)
                    current_bundle.push(idx);
                } else {
                    // Different tip recipient or bundle full - start new bundle
                    // Finalize current bundle if it has 2+ transactions
                    if current_bundle.len() >= 2 {
                        bundle_clusters.push(current_bundle.clone());
                    }
                    current_bundle = vec![idx];
                    current_tip_recipient = Some(tx.tip_recipient.clone());
                }
            } else {
                // Start new bundle
                current_bundle = vec![idx];
                current_tip_recipient = Some(tx.tip_recipient.clone());
            }
        } else {
            // Transaction without tip - we cannot reliably determine if it's part of a bundle
            // Finalize current bundle if it has 2+ transactions
            if current_bundle.len() >= 2 {
                bundle_clusters.push(current_bundle.clone());
            }
            current_bundle.clear();
            current_tip_recipient = None;
        }
    }
    
    // Finalize last bundle if exists
    if current_bundle.len() >= 2 {
        bundle_clusters.push(current_bundle);
    }
    
    // Find largest bundle size
    let largest_bundle_size = bundle_clusters
        .iter()
        .map(|bundle| bundle.len())
        .max()
        .unwrap_or(0) as u32;

    // Analyze tips from all transactions to identify landing services
    let mut tip_recipient_counts: HashMap<String, u32> = HashMap::new();
    
    for tx in transactions {
        if !tx.tip_recipient.is_empty() && tx.tip_amount > 0 {
            *tip_recipient_counts.entry(tx.tip_recipient.clone()).or_insert(0) += 1;
        }
    }
    
    // Identify landing service from tip patterns
    // If we see transactions sending tips to the same address, that's likely a landing service
    let mut landing_service_found = String::new();
    let mut landing_service_count = 0u32;
    
    if let Some((recipient, count)) = tip_recipient_counts.iter().max_by_key(|(_, &c)| c) {
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

    BundlingAnalysisRow {
        slot,
        blockhash,
        block_time,
        largest_bundle_size,
        validator_key,
        landing_service: landing_service_found,
        landing_service_count,
    }
}

