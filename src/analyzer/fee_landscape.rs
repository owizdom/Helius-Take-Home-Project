use clickhouse::Row;
use serde::{Deserialize, Serialize};
use super::types::Transaction;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct FeeLandscapeRow {
    pub slot: u64,
    pub block_time: u64,
    pub fee_avg: f64,
    pub compute_budget_percent: f32,
}

pub fn analyze_fee_landscape(
    transactions: &[Transaction],
    slot: u64,
    block_time: u64,
) -> FeeLandscapeRow {
    let fees: Vec<u64> = transactions.iter().map(|t| t.fee).collect();
    let fee_avg = fees.iter().sum::<u64>() as f64 / fees.len() as f64;

    // ComputeBudget analysis
    let compute_budget_txs: Vec<&Transaction> = transactions.iter().filter(|t| t.has_compute_budget == 1).collect();
    let compute_budget_count = compute_budget_txs.len() as u32;
    let compute_budget_percent = (compute_budget_count as f32 / transactions.len() as f32) * 100.0;

    FeeLandscapeRow {
        slot,
        block_time,
        fee_avg,
        compute_budget_percent,
    }
}

