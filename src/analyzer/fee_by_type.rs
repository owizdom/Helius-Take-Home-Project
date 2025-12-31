use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::Transaction;
use super::utils::get_transaction_type;

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct FeeByTransactionTypeRow {
    pub slot: u64,
    pub block_time: u64,
    pub transaction_type: String,
    pub transaction_count: u32,
    pub total_fee: u64,
}

pub fn analyze_fee_by_type(
    transactions: &[Transaction],
    slot: u64,
    block_time: u64,
) -> Vec<FeeByTransactionTypeRow> {
    let mut type_fees: HashMap<String, Vec<(u64, u8, u8)>> = HashMap::new(); // type -> (fee, failed, has_compute_budget)

    for tx in transactions {
        let tx_type = get_transaction_type(tx);
        type_fees.entry(tx_type).or_insert_with(Vec::new).push((tx.fee, tx.failed, tx.has_compute_budget));
    }

    let mut fee_by_type_rows = Vec::new();
    for (tx_type, fee_data) in type_fees {
        let count = fee_data.len() as u32;
        let fees: Vec<u64> = fee_data.iter().map(|(f, _, _)| *f).collect();
        let total_fee = fees.iter().sum::<u64>();

        fee_by_type_rows.push(FeeByTransactionTypeRow {
            slot,
            block_time,
            transaction_type: tx_type,
            transaction_count: count,
            total_fee,
        });
    }

    fee_by_type_rows
}

