use solana_block_fetcher::analyzer::bundling::analyze_bundling;
use solana_block_fetcher::analyzer::types::Transaction;

#[test]
fn test_blockhash_clustering() {
    // Create 150 transactions with same blockhash
    let mut transactions = Vec::new();
    let bundle_blockhash = "bundle123".to_string();
    
    for i in 0..150 {
        transactions.push(Transaction {
            slot: 1000,
            position: i + 1,
            signature: format!("sig{}", i),
            recent_blockhash: bundle_blockhash.clone(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec!["JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string()],
            landing_service: String::new(),
            tip_recipient: String::new(),
            tip_amount: 0,
        });
    }
    
    let result = analyze_bundling(&transactions, 1000, "blockhash".to_string(), 1234567890, "validator1".to_string());
    
    assert_eq!(result.unique_blockhashes, 1);
    assert_eq!(result.largest_blockhash_group, 150);
}

#[test]
fn test_multiple_blockhashes() {
    // Create transactions with multiple blockhashes
    let mut transactions = Vec::new();
    
    // 120 transactions with blockhash1
    for i in 0..120 {
        transactions.push(Transaction {
            slot: 1000,
            position: i + 1,
            signature: format!("sig{}", i),
            recent_blockhash: "blockhash1".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: String::new(),
            tip_amount: 0,
        });
    }
    
    // 30 transactions with blockhash2
    for i in 0..30 {
        transactions.push(Transaction {
            slot: 1000,
            position: 120 + i + 1,
            signature: format!("sig{}", 120 + i),
            recent_blockhash: "blockhash2".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: String::new(),
            tip_amount: 0,
        });
    }
    
    let result = analyze_bundling(&transactions, 1000, "blockhash".to_string(), 1234567890, "validator1".to_string());
    
    assert_eq!(result.unique_blockhashes, 2);
    assert_eq!(result.largest_blockhash_group, 120);
}

