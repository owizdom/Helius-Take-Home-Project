use solana_block_fetcher::analyzer::bundling::analyze_bundling;
use solana_block_fetcher::analyzer::types::Transaction;

#[test]
fn test_bundle_detection_same_tip_recipient() {
    // Create a bundle: 3 sequential transactions with same tip recipient
    let jito_tip = "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt".to_string();
    let transactions = vec![
        Transaction {
            slot: 1000,
            position: 1,
            signature: "sig1".to_string(),
            recent_blockhash: "bh1".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: jito_tip.clone(),
            tip_amount: 10000,
        },
        Transaction {
            slot: 1000,
            position: 2,
            signature: "sig2".to_string(),
            recent_blockhash: "bh2".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: jito_tip.clone(),
            tip_amount: 10000,
        },
        Transaction {
            slot: 1000,
            position: 3,
            signature: "sig3".to_string(),
            recent_blockhash: "bh3".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: jito_tip.clone(),
            tip_amount: 10000,
        },
    ];
    
    let result = analyze_bundling(&transactions, 1000, "blockhash".to_string(), 1234567890, "validator1".to_string());
    
    // Should detect bundle of size 3
    assert_eq!(result.largest_bundle_size, 3);
}

#[test]
fn test_no_bundles_no_tips() {
    // Transactions without tips - no bundles should be detected
    let transactions = vec![
        Transaction {
            slot: 1000,
            position: 1,
            signature: "sig1".to_string(),
            recent_blockhash: "bh1".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: String::new(),
            tip_amount: 0,
        },
        Transaction {
            slot: 1000,
            position: 2,
            signature: "sig2".to_string(),
            recent_blockhash: "bh2".to_string(),
            fee: 5000,
            failed: 0,
            has_compute_budget: 0,
            is_vote: 0,
            is_system: 0,
            program_ids: vec![],
            landing_service: String::new(),
            tip_recipient: String::new(),
            tip_amount: 0,
        },
    ];
    
    let result = analyze_bundling(&transactions, 1000, "blockhash".to_string(), 1234567890, "validator1".to_string());
    
    // No bundles detected (need at least 2 transactions with tips)
    assert_eq!(result.largest_bundle_size, 0);
}

