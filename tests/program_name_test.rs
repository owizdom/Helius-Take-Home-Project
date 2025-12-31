use solana_block_fetcher::analyzer::utils::get_program_name;

#[test]
fn test_program_name_mapping_known_programs() {
    // Test critical program mappings
    assert_eq!(get_program_name("11111111111111111111111111111111"), "System Program");
    assert_eq!(get_program_name("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"), "Jupiter");
    assert_eq!(get_program_name("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"), "Raydium AMM");
    assert_eq!(get_program_name("CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"), "Raydium CLMM");
    assert_eq!(get_program_name("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"), "SPL Token");
    assert_eq!(get_program_name("ComputeBudget111111111111111111111111111111"), "Compute Budget");
    assert_eq!(get_program_name("Vote111111111111111111111111111111111111111"), "Vote Program");
}

#[test]
fn test_program_name_unknown_program() {
    // Unknown program should return the ID itself
    let unknown_id = "UnknownProgram123456789012345678901234567890";
    assert_eq!(get_program_name(unknown_id), unknown_id);
}

#[test]
fn test_program_name_pattern_matching() {
    // Test pattern-based matching
    assert_eq!(get_program_name("Vote111111111111111111111111111111111111111XYZ"), "Vote Program");
    assert_eq!(get_program_name("ComputeBudgetABC"), "Compute Budget");
    assert_eq!(get_program_name("BPFLoader2111111111111111111111111111111111"), "BPF Loader");
}

#[test]
fn test_program_name_dex_protocols() {
    // Test DEX protocol mappings
    assert_eq!(get_program_name("9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin"), "Serum DEX v3");
    assert_eq!(get_program_name("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"), "OpenBook");
    assert_eq!(get_program_name("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc"), "Orca");
    assert_eq!(get_program_name("9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP"), "Orca Swap 2");
}

#[test]
fn test_program_name_spl_programs() {
    // Test SPL program mappings
    assert_eq!(get_program_name("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"), "SPL Token-2022");
    assert_eq!(get_program_name("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"), "Associated Token Account");
}

