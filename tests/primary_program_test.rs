use solana_block_fetcher::analyzer::utils::get_primary_program;

#[test]
fn test_primary_program_filters_system_programs() {
    // Transaction with system program first, then Jupiter
    let program_ids = vec![
        "11111111111111111111111111111111".to_string(), // System
        "ComputeBudget111111111111111111111111111111".to_string(), // Compute Budget
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(), // Jupiter
    ];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return Jupiter (first non-system, non-compute-budget)
    assert_eq!(primary, Some("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string()));
}

#[test]
fn test_primary_program_only_system_programs() {
    // Transaction with only system/compute budget programs
    let program_ids = vec![
        "11111111111111111111111111111111".to_string(), // System
        "ComputeBudget111111111111111111111111111111".to_string(), // Compute Budget
    ];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return first one (system) since all are filtered
    assert_eq!(primary, Some("11111111111111111111111111111111".to_string()));
}

#[test]
fn test_primary_program_empty_list() {
    // Edge case: empty program list
    let program_ids = vec![];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return None
    assert_eq!(primary, None);
}

#[test]
fn test_primary_program_direct_non_system() {
    // Transaction starting with non-system program
    let program_ids = vec![
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(), // Jupiter
        "11111111111111111111111111111111".to_string(), // System
    ];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return Jupiter (first non-system)
    assert_eq!(primary, Some("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string()));
}

#[test]
fn test_primary_program_multiple_non_system() {
    // Transaction with multiple non-system programs
    let program_ids = vec![
        "11111111111111111111111111111111".to_string(), // System
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(), // Jupiter
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8".to_string(), // Raydium
    ];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return first non-system program (Jupiter)
    assert_eq!(primary, Some("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string()));
}

#[test]
fn test_primary_program_vote_program() {
    // Transaction with vote program (should be filtered as "Vote" type)
    let program_ids = vec![
        "11111111111111111111111111111111".to_string(), // System
        "Vote111111111111111111111111111111111111111".to_string(), // Vote
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4".to_string(), // Jupiter
    ];
    
    let primary = get_primary_program(&program_ids);
    
    // Should return Jupiter (Vote is not System or ComputeBudget, but we filter by type)
    // Actually, looking at the code, it filters by type "System" and "ComputeBudget"
    // Vote has type "Vote", so it should be returned first
    // But wait, let me check the logic again - it filters if prog_type == "System" || prog_type == "ComputeBudget"
    // So Vote should pass through... but the function returns the first non-system, non-compute-budget
    // So it should return Vote, not Jupiter
    // Actually, I need to check what categorize_program_type returns for Vote
    // It returns ("Vote", program_id) for Vote programs
    // So Vote is not "System" or "ComputeBudget", so it should be returned
    assert_eq!(primary, Some("Vote111111111111111111111111111111111111111".to_string()));
}

