use clickhouse::Row;
use serde::{Deserialize, Serialize};
use super::types::Transaction;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct FeeLandscapeRow {
    pub slot: u64,
    pub block_time: u64,
    pub fee_avg: f64,
    pub compute_budget_percent: f32,
    pub fee_ordering_correlation: f32,
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

    // Fee ordering analysis
    let actual_fees: Vec<u64> = transactions.iter().map(|t| t.fee).collect();
    
    // Simple correlation (position vs fee rank)
    let mut fee_rank_correlation = 0.0;
    if fees.len() > 1 {
        let sorted_indices: Vec<usize> = {
            let mut indices: Vec<(usize, u64)> = actual_fees.iter().enumerate().map(|(i, &f)| (i, f)).collect();
            indices.sort_by(|a, b| b.1.cmp(&a.1));
            indices.iter().map(|(i, _)| *i).collect()
        };
        let perfect_order: Vec<usize> = (0..fees.len()).collect();
        let diff: usize = sorted_indices.iter().zip(perfect_order.iter()).map(|(a, b)| a.abs_diff(*b)).sum();
        fee_rank_correlation = 1.0 - (diff as f32 / (fees.len() * fees.len() / 2) as f32);
    }

    FeeLandscapeRow {
        slot,
        block_time,
        fee_avg,
        compute_budget_percent,
        fee_ordering_correlation: fee_rank_correlation.max(0.0).min(1.0),
    }
}

