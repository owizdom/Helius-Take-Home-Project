use super::types::Transaction;

#[allow(dead_code)]
pub fn calculate_percentile(mut fees: Vec<u64>, percentile: f64) -> u64 {
    fees.sort();
    let index = ((fees.len() as f64) * percentile / 100.0) as usize;
    fees.get(index.min(fees.len() - 1)).copied().unwrap_or(0)
}

#[allow(dead_code)]
pub fn calculate_std_dev(fees: &[u64], mean: f64) -> f64 {
    let variance: f64 = fees.iter().map(|&f| (f as f64 - mean).powi(2)).sum::<f64>() / fees.len() as f64;
    variance.sqrt()
}

pub fn categorize_program_type(program_id: &str) -> (&'static str, &str) {
    // Well-known Solana program IDs
    match program_id {
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => ("SPL Token", program_id),
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4" => ("Jupiter", program_id),
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => ("Raydium", program_id),
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => ("Orca", program_id),
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => ("Orca Whirlpool", program_id),
        "11111111111111111111111111111111" => ("System", program_id),
        p if p.contains("Vote111111111111111111111111111111111111111") => ("Vote", program_id),
        p if p.contains("ComputeBudget") => ("ComputeBudget", program_id),
        p if p.contains("Stake") => ("Staking", program_id),
        _ => ("Other", program_id),
    }
}

pub fn get_program_name(program_id: &str) -> String {
    match program_id {
        // System Programs
        "11111111111111111111111111111111" => "System Program".to_string(),
        "BPFLoader2111111111111111111111111111111111" => "BPF Loader".to_string(),
        "BPFLoaderUpgradeab1e11111111111111111111111" => "Upgradeable Loader".to_string(),
        "ComputeBudget111111111111111111111111111111" => "Compute Budget".to_string(),
        "AddressLookupTab1e1111111111111111111111111" => "Address Lookup Table".to_string(),
        "Stake11111111111111111111111111111111111111" => "Stake Program".to_string(),
        "Vote111111111111111111111111111111111111111" => "Vote Program".to_string(),
        "SysvarRent111111111111111111111111111111111" => "Rent Program".to_string(),
        "SysvarC1ock11111111111111111111111111111111" => "Clock Sysvar".to_string(),
        "Sysvar1nstructions1111111111111111111111111" => "Instructions Sysvar".to_string(),
        "Config1111111111111111111111111111111111111" => "Config".to_string(),
        "Ed25519SigVerify111111111111111111111111111" => "Ed25519".to_string(),
        "KeccakSecp256k11111111111111111111111111111" => "Secp256k1".to_string(),
        "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr" => "Memo".to_string(),
        "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo" => "Memo 1".to_string(),
        "Feat1YXHhH6t1juaWF74WLcfv4XoNocjXA6sPWHNgAse" => "Feature Proposal".to_string(),
        "ProgM6JCCvbYkfKqJYHePx4xxSUSqJp7rh8Lyv7nk7S" => "Program Metadata".to_string(),
        "SySTEM1eSU2p4BGQfQpimFEWWSC1XDFeun3Nqzz3rT7" => "ZK Light System Program".to_string(),
        
        // SPL Programs
        "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" => "SPL Token".to_string(),
        "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb" => "SPL Token-2022".to_string(),
        "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL" => "Associated Token Account".to_string(),
        
        // DEX Protocols
        "9xQeWvG816bUx9EPjHmaT23yvVM2ZWbrrpZb9PusVFin" => "Serum DEX v3".to_string(),
        "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX" => "OpenBook".to_string(),
        "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8" => "Raydium AMM".to_string(),
        "RVKd61ztZW9Gd3yx9wD6iFYkPHC3g4JbbmB6z9zK4pB" => "Raydium LP 1".to_string(),
        "27haf8L6oxUeXrHrgEgsexjSY5hbVUWEmvv9Nyxg8vQv" => "Raydium LP 2".to_string(),
        "EhhTKczWMGQt46ynNeRX1WfeagwwJd7ufHvCDjRxjo5Q" => "Raydium Staking".to_string(),
        "9HzJyW1qZsEiSfMUf6L2jo3CcTKAyBmSyKdwQeYisHrC" => "Raydium IDO".to_string(),
        "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK" => "Raydium CLMM".to_string(),
        "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc" => "Orca".to_string(),
        "9W959DqEETiGZocYWCQPaJ6sBmUzgfxXfqGeTEdp3aQP" => "Orca Swap 2".to_string(),
        "DjVE6JNiYqPL2QXyCUUh8rNjHrbz9hXHNYt99MQ59qw1" => "Orca Swap 1".to_string(),
        "82yxjeMsvaURa4MbZZ7WZZHfobirZYkH1zF8fmeGtyaQ" => "Orca Aquafarm".to_string(),
        "PhoeNiXZ8ByJGLkxNfZRnkUfjvmuYqLR89jjFHGqdXY" => "Phoenix Orderbook".to_string(),
        "SSwpkEEcbUqx4vtoEByFjSkhKdCT862DNVb52nZg1UZ" => "Saber Swap".to_string(),
        "Crt7UoUR6QgrFrN7j8rmSQpUTNWNSitSwWvsWGf1qZ5t" => "Saber Router".to_string(),
        "SSwpMgqNDsyV7mAgN9ady4bDVu5ySjmmXejXvy2vLt1" => "Step Swap".to_string(),
        "Dooar9JkhdZ7J3LHN3A7YCuoGRUggXhQaG4kijfLGU2j" => "STEPN DEX".to_string(),
        "SWiMDJYFUGj6cPrQ6QYYYWZtvXQdRChSVAygDZDsCHC" => "Swim Swap".to_string(),
        "SwaPpA9LAaLfeLi3a68M4DjnLqgtticKg6CnyNwgAC8" => "Swap".to_string(),
        "MERLuDFBMmsHnsBPZw2sDQZHvXFMwp8EdjudcU2HKky" => "Mercurial".to_string(),
        "JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4" => "Jupiter".to_string(),
        
        // Lending Protocols
        "So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo" => "Solend".to_string(),
        "LendZqTs7gn5CTSJU1jWKhKuVpjJGom45nnwPb2AMTi" => "Lending".to_string(),
        "Port7uDYB3wk6GJAw4KT1WpTeMtSu9bTcChBHkX2LfR" => "Port Finance".to_string(),
        
        // NFT/Metaplex
        "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s" => "Metaplex Metadata".to_string(),
        "cndyAnrLdpjq1Ssp1z8xxDsB8dxe7u4HL5Nxi2K5WXZ" => "Candy Machine".to_string(),
        "cndy3Z4yapfJBmL3ShUp5exZKqR3z33thTzeNMm2gRZ" => "Candy Machine v2".to_string(),
        "p1exdMJcjVao65QdewkaZRUnU6VPSXhus9n2GzWfh98" => "Metaplex".to_string(),
        "auctxRXPeJoc4817jDhf4HbjnhEcr1cCXenosMhK5R8" => "NFT Auction".to_string(),
        "vau1zxA2LbssAUEF7Gpw91zMM1LvXrvpzJtmZ58rPsn" => "Token Vault".to_string(),
        "namesLPneVptA9Z5rqUDD9tMTWEJwofgaYwp8cawRkX" => "Name Service".to_string(),
        
        // Other Protocols
        "DtmE9D2CSB4L5D6A15mraeEjrGMm6auWVzgaD8hK2tZM" => "Switchboard".to_string(),
        "cjg3oHmg9uuPsP8D6g29NWvhySJkdYdAo9D25PRbKXJ" => "Chainlink Oracle".to_string(),
        "Gt9S41PtjR58CbG9JhJ3J6vxesqrNAswbWYbLNTMZA3c" => "Chainlink Data Streams Verifier".to_string(),
        "HEvSKofvBgfaexv23kMabbYqxasxU3mQ4ibBMEmJWHny" => "Chainlink Store".to_string(),
        "FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH" => "Pyth Mainnet".to_string(),
        "8tfDNiaEyrV6Q1U4DEXrEigs9DoDtkugzFbybENEbCDz" => "Pyth Testnet".to_string(),
        "gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s" => "Pyth Devnet".to_string(),
        "CrX7kMhLC3cSsXJdT7JDgqrRVWGnUpX3gfEfxxU2NVLi" => "Solido".to_string(),
        "MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD" => "Marinade".to_string(),
        "SPoo1Ku8WFXoNDMHPsrGSTSG1Y47rzgn41SLUNakuHy" => "Stake Pool".to_string(),
        "WormT3McKhFJ2RkiGpdw9GKvNCrB2aB54gb2uV9MfQC" => "Wormhole".to_string(),
        "worm2ZoG2kUd4vFXhvjh93UUH596ayRfgQ2MgjNMTth" => "Wormhole Core".to_string(),
        "wormDTUJ6AWPNvk59vGQbDvGJmqbDTdgWgAqcLBCgUb" => "Wormhole Token".to_string(),
        "3u8hJUVTA4jH1wYAyUur7FFZVQ8H635K3tSHHF4ssjQ5" => "Wormhole Core (Devnet)".to_string(),
        "DZnkkTmCiFWfYTfT41X3Rd1kDgozqzxWaHqsw6W4x2oe" => "Wormhole Token (Devnet)".to_string(),
        "2rHhojZ7hpu1zA91nvZmT8TqWWvMcKmmNBCr2mKTtMq4" => "Wormhole NFT (Devnet)".to_string(),
        "WnFt12ZrnzZrFZkt2xsNsaNWoQribnuQ5B5FrDbwDhD" => "Wormhole NFT".to_string(),
        "C64kTdg1Hzv5KoQmZrQRcm2Qz7PkxtFBgw7EpFhvYn8W" => "Acumen".to_string(),
        "CJsLwbP1iu5DuUikHEJnLfANgKy6stB2uFgvBBHoyxwz" => "Solanart".to_string(),
        "5ZfZAwP2m93waazg8DkrrVmsupeiPEvaEHowiUP7UAbJ" => "Solanart Go".to_string(),
        "DF1ow4tspfHX9JwWJsAb9epbkA8hmpSEAtxXy1V27QBH" => "DFlow Aggregator 4".to_string(),
        "3XXuUFfweXBwFgFfYaejLvZE4cGZiHgKiGfMtdxNzYmv" => "Clockwork 1".to_string(),
        "CLoCKyJ6DXBJqqu2VWx9RLbgnwwR6BMHHuyasVmfMzBh" => "Clockwork 2".to_string(),
        "VRFzZoJdhFWL8rkvu87LpKM3RbcVezpMEc6X5GVDr7y" => "ORAO VRF 2".to_string(),
        "oreV2ZymfyeXgNgBdqMkumTqqAprVqgBWQfoYkrtKWQ" => "ORE".to_string(),
        "BrEAK7zGZ6dM71zUDACDqJnekihmwF15noTddWTsknjC" => "Break Solana".to_string(),
        "L2TExMFKdjpN9kozasaurPirfHy9P8sbXoAN1qA3S95" => "Lighthouse Program".to_string(),
        "22Y43yTVxuUkoRKdm9thyRhQ3SdgQS7c7kB6UNCiaczD" => "Serum Swap".to_string(),
        "BJ3jrUzddfuSrZHXSCxMUUQsjKEyLmuuyZebkcaFp2fg" => "Serum 1".to_string(),
        "EUqojwWA2rd19FZrzeBncJsm38Jm1hEhE3zsmX3bRc2o" => "Serum 2".to_string(),
        "WvmTNLpGMVbwJVYztYL4Hnsy82cJhQorxjnnXcRm3b6" => "Serum Pool".to_string(),
        "22zoJMtdu4tQc2PzL74ZUT7FrwgB1Udec8DdW4yw4BdG" => "SAS Program".to_string(),
        "JD3bq9hGdy38PuWQ4h2YJpELmHVGPPfFSuFkpzAd9zfu" => "Mango 1".to_string(),
        "5fNfvyp5czQVX77yoACa3JJVEhdRaWjPuazuWgjhTqEH" => "Mango 2".to_string(),
        "mv3ekLzLbnVPNxjSKvqBpU3ZeZXPQdEC3bp5MDEBG68" => "Mango 3".to_string(),
        "MangoCzJ36AjZf5TTdFJcJx8xgfZ9jGmG9xS8YyATXxP" => "Mango v4".to_string(),
        "GqTPL6qRf5aUuqscLh8Rg2HTxPUXfhhAXDptTLhp1t2J" => "Mango Governance".to_string(),
        "7sPptkymzvayoSbLXzBsXEF8TSf3typNnAWkrKrDizNb" => "Mango ICO".to_string(),
        "QMNeHCGYnLVDn1icRAfQZpjPLBNkfGbSKRB83G5d8KB" => "Quarry Mine".to_string(),
        "QMMD16kjauP5knBwxNUJRZ1Z5o3deBuFrqVjBVmmqto" => "Quarry Merge Mine".to_string(),
        "QMWoBmAyJLAsA1Lh9ugMTw2gciTihncciphzdNzdZYV" => "Quarry Mint Wrapper".to_string(),
        "QRDxhMw1P2NEfiw5mYXG79bwfgHTdasY2xNP76XSea9" => "Quarry Redeemer".to_string(),
        "QREGBnEj9Sa5uR91AV8u3FxThgP5ZCvdZUW2bHAkfNc" => "Quarry Registry".to_string(),
        "A5JxZVHgXe7fn5TqJXm6Hj2zKh1ptDapae2YjtXZJoy" => "Finterest Token Manager".to_string(),
        "CmFuqQTLs2nQof5uaktJn1a6k2VdbGmZPfrJufB2Vm3F" => "Finterest User Manager".to_string(),
        "cTokenmWW8bLPjZEBAUgYy3zKxQZW6VKi7bqNFEVv3m" => "ZK Compressed Token Program".to_string(),
        "compr6CUsB5m2jS4Y3831ztGSTnDpnKJTKS95d64XVq" => "ZK Account Compression Program".to_string(),
        "cmtDvXumGCrqC1Age74AVPhSRVXJMd8PJS91L8KbNCK" => "Account Compression".to_string(),
        
        // Pattern matching for partial matches
        p if p.contains("Vote111111111111111111111111111111111111111") => "Vote Program".to_string(),
        p if p.contains("ComputeBudget") => "Compute Budget".to_string(),
        p if p.contains("Stake") && !p.starts_with("Stake11111111111111111111111111111111111111") => "Staking".to_string(),
        p if p.contains("BPFLoader") => "BPF Loader".to_string(),
        p if p.starts_with("Sysvar") => "Sysvar".to_string(),
        p if p.starts_with("JUP") || p.contains("Jupiter") => "Jupiter".to_string(),
        
        _ => program_id.to_string(), // Return the ID if not found
    }
}

pub fn get_primary_program(program_ids: &[String]) -> Option<String> {
    // Get the first non-system, non-compute-budget program
    for prog_id in program_ids {
        let (prog_type, _) = categorize_program_type(prog_id);
        if prog_type != "System" && prog_type != "ComputeBudget" {
            return Some(prog_id.clone());
        }
    }
    // If all are system/compute budget, return the first one
    program_ids.first().cloned()
}

pub fn get_transaction_type(transaction: &Transaction) -> String {
    if transaction.is_vote == 1 {
        "vote".to_string()
    } else if transaction.is_system == 1 {
        "system".to_string()
    } else if let Some(primary_program) = get_primary_program(&transaction.program_ids) {
        let (prog_type, _) = categorize_program_type(&primary_program);
        match prog_type {
            "SPL Token" => "spl_token".to_string(),
            "Jupiter" => "jupiter".to_string(),
            "Raydium" => "raydium".to_string(),
            "Orca" | "Orca Whirlpool" => "orca".to_string(),
            _ => "other".to_string(),
        }
    } else {
        "other".to_string()
    }
}

