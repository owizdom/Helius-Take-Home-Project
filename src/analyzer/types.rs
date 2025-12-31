#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Transaction {
    pub slot: u64,
    pub position: u32,
    pub signature: String,
    pub recent_blockhash: String,
    pub fee: u64,
    pub failed: u8,
    pub has_compute_budget: u8,
    pub is_vote: u8,
    pub is_system: u8,
    pub program_ids: Vec<String>,
    pub landing_service: String,  // Empty string if unknown, otherwise service name
    pub tip_recipient: String,   // Address that received tip (if any)
    pub tip_amount: u64,          // Tip amount in lamports (0 if no tip)
}

