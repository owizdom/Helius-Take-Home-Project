use clickhouse::Row;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use super::types::Transaction;
use super::utils::{categorize_program_type, get_primary_program, get_program_name};

#[derive(Debug, Row, Serialize, Deserialize)]
pub struct ProgramFeeAnalysisRow {
    pub slot: u64,
    pub block_time: u64,
    pub program_type: String,
    pub program_name: String,
    pub transaction_count: u32,
    pub total_fee: u64,
    pub min_fee: u64,
    pub max_fee: u64,
}

pub fn analyze_program_fees(
    transactions: &[Transaction],
    slot: u64,
    block_time: u64,
) -> Vec<ProgramFeeAnalysisRow> {
    // Track fees by program type
    let mut program_fees: HashMap<(String, String), Vec<u64>> = HashMap::new(); // (program_type, program_id) -> fees

    for tx in transactions {
        if let Some(primary_program) = get_primary_program(&tx.program_ids) {
            let (prog_type, _) = categorize_program_type(&primary_program);
            let key = (prog_type.to_string(), primary_program);
            program_fees.entry(key).or_insert_with(Vec::new).push(tx.fee);
        }
    }

    // Create program fee analysis rows
    let mut program_fee_rows = Vec::new();
    for ((prog_type, prog_id), fees) in program_fees {
        let count = fees.len() as u32;
        let total_fee = fees.iter().sum::<u64>();
        let min_fee = *fees.iter().min().unwrap_or(&0);
        let max_fee = *fees.iter().max().unwrap_or(&0);

        let program_name = get_program_name(&prog_id);
        program_fee_rows.push(ProgramFeeAnalysisRow {
            slot,
            block_time,
            program_type: prog_type.clone(),
            program_name,
            transaction_count: count,
            total_fee,
            min_fee,
            max_fee,
        });
    }

    // Sort by total_fee descending to see what's paying the most
    program_fee_rows.sort_by(|a, b| b.total_fee.cmp(&a.total_fee));

    program_fee_rows
}

