use solana_block_fetcher::analyzer::fee_landscape::analyze_fee_landscape;
use solana_block_fetcher::analyzer::types::Transaction;

#[test]
fn test_fee_ordering_perfect_correlation() {
    // Transactions ordered by fee (highest first) = perfect correlation
    let transactions = vec![
        Transaction { slot: 1000, position: 1, signature: "sig1".to_string(), recent_blockhash: "bh1".to_string(), fee: 10000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 2, signature: "sig2".to_string(), recent_blockhash: "bh2".to_string(), fee: 8000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 3, signature: "sig3".to_string(), recent_blockhash: "bh3".to_string(), fee: 5000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 4, signature: "sig4".to_string(), recent_blockhash: "bh4".to_string(), fee: 2000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
    ];
    
    let result = analyze_fee_landscape(&transactions, 1000, 1234567890);
    
    // Should have high correlation (close to 1.0)
    assert!(result.fee_ordering_correlation > 0.8);
    assert_eq!(result.fee_avg, 6250.0);
}

#[test]
fn test_fee_ordering_reverse_correlation() {
    // Transactions ordered by fee (lowest first) = low correlation
    let transactions = vec![
        Transaction { slot: 1000, position: 1, signature: "sig1".to_string(), recent_blockhash: "bh1".to_string(), fee: 1000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 2, signature: "sig2".to_string(), recent_blockhash: "bh2".to_string(), fee: 2000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 3, signature: "sig3".to_string(), recent_blockhash: "bh3".to_string(), fee: 5000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 4, signature: "sig4".to_string(), recent_blockhash: "bh4".to_string(), fee: 10000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
    ];
    
    let result = analyze_fee_landscape(&transactions, 1000, 1234567890);
    
    // Should have low correlation (validator not following fee-based ordering)
    assert!(result.fee_ordering_correlation < 0.5);
    assert_eq!(result.fee_avg, 4500.0); // (1000 + 2000 + 5000 + 10000) / 4 = 4500
}

#[test]
fn test_fee_ordering_single_transaction() {
    // Edge case: single transaction
    let transactions = vec![
        Transaction { slot: 1000, position: 1, signature: "sig1".to_string(), recent_blockhash: "bh1".to_string(), fee: 5000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
    ];
    
    let result = analyze_fee_landscape(&transactions, 1000, 1234567890);
    
    // Should handle gracefully (correlation = 0 for single tx)
    assert_eq!(result.fee_ordering_correlation, 0.0);
    assert_eq!(result.fee_avg, 5000.0);
}

#[test]
fn test_fee_ordering_compute_budget_percentage() {
    // Test compute budget usage calculation
    let transactions = vec![
        Transaction { slot: 1000, position: 1, signature: "sig1".to_string(), recent_blockhash: "bh1".to_string(), fee: 5000, failed: 0, has_compute_budget: 1, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 2, signature: "sig2".to_string(), recent_blockhash: "bh2".to_string(), fee: 5000, failed: 0, has_compute_budget: 1, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 3, signature: "sig3".to_string(), recent_blockhash: "bh3".to_string(), fee: 5000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
        Transaction { slot: 1000, position: 4, signature: "sig4".to_string(), recent_blockhash: "bh4".to_string(), fee: 5000, failed: 0, has_compute_budget: 0, is_vote: 0, is_system: 0, program_ids: vec![], landing_service: String::new(), tip_recipient: String::new(), tip_amount: 0 },
    ];
    
    let result = analyze_fee_landscape(&transactions, 1000, 1234567890);
    
    // 2 out of 4 transactions have compute budget = 50%
    assert_eq!(result.compute_budget_percent, 50.0);
}

#[test]
fn test_fee_ordering_empty_transactions() {
    // Edge case: empty transaction list
    // Division by zero in Rust for floats returns Infinity or NaN
    let transactions = vec![];
    
    let result = analyze_fee_landscape(&transactions, 1000, 1234567890);
    
    // Should handle gracefully - fee_avg will be NaN or Inf, correlation will be 0
    assert_eq!(result.slot, 1000);
    assert_eq!(result.block_time, 1234567890);
    assert!(result.fee_avg.is_nan() || result.fee_avg.is_infinite());
    assert_eq!(result.fee_ordering_correlation, 0.0);
    assert!(result.compute_budget_percent.is_nan() || result.compute_budget_percent.is_infinite());
}

