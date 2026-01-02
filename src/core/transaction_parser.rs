use crate::analyzer::types::Transaction;
use crate::analyzer::landing_services::identify_landing_service;
use solana_transaction_status::{EncodedTransaction, UiInstruction, UiMessage, UiParsedInstruction};

pub fn parse_transactions(
    block_transactions: &[solana_transaction_status::EncodedTransactionWithStatusMeta],
    slot: u64,
) -> Vec<Transaction> {
    let mut transactions = Vec::new();
    
    for (idx, transaction_with_meta) in block_transactions.iter().enumerate() {
        let position = (idx + 1) as u32;
        
        let (recent_blockhash, fee, failed, has_compute_budget, is_vote, is_system, program_ids, landing_service, tip_recipient, tip_amount) = match &transaction_with_meta.transaction {
            EncodedTransaction::Json(transaction_json) => {
                let message = &transaction_json.message;
                
                let recent_blockhash = match message {
                    UiMessage::Parsed(parsed_message) => parsed_message.recent_blockhash.clone(),
                    UiMessage::Raw(raw_message) => raw_message.recent_blockhash.clone(),
                };
                
                let mut landing_service_found = String::new();
                let mut tip_recipient = String::new();
                let mut tip_amount = 0u64;
                
                let program_ids: Vec<String> = match message {
                    UiMessage::Parsed(parsed_message) => {
                        parsed_message.instructions.iter().map(|inst| {
                            match inst {
                                UiInstruction::Parsed(parsed_instruction) => {
                                    match parsed_instruction {
                                        UiParsedInstruction::Parsed(instruction_parsed) => {
                                            // Check if this is a System Program transfer to a known tip account
                                            if instruction_parsed.program_id == "11111111111111111111111111111111" {
                                                // The parsed field is already a Value, try to extract transfer info
                                                if let Some(parsed_data) = instruction_parsed.parsed.as_object() {
                                                    // Check if it's a transfer instruction
                                                    if let Some(inst_type) = parsed_data.get("type").and_then(|v| v.as_str()) {
                                                        if inst_type == "transfer" {
                                                            // Extract destination address and amount
                                                            if let Some(info) = parsed_data.get("info").and_then(|v| v.as_object()) {
                                                                if let Some(destination) = info.get("destination").and_then(|v| v.as_str()) {
                                                                    // Extract tip amount (lamports)
                                                                    if let Some(lamports) = info.get("lamports").and_then(|v| v.as_u64()) {
                                                                        // Set tip_recipient for ANY System Program transfer (not just known tip accounts)
                                                                        // This allows us to detect unknown landing services
                                                                        if tip_recipient.is_empty() && lamports > 0 {
                                                                            tip_recipient = destination.to_string();
                                                                            tip_amount = lamports;
                                                                        }
                                                                        
                                                                        // Check if this is a known landing service address
                                                                        if let Some(service) = identify_landing_service(destination) {
                                                                            if landing_service_found.is_empty() {
                                                                                landing_service_found = service;
                                                                            }
                                                                        } else if !tip_recipient.is_empty() && tip_recipient == destination {
                                                                            // Unknown landing service - set to "Unknown: {address}" format
                                                                            // This matches the format used in bundling.rs
                                                                            if landing_service_found.is_empty() {
                                                                                landing_service_found = format!("Unknown: {}", destination);
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            instruction_parsed.program_id.clone()
                                        }
                                        UiParsedInstruction::PartiallyDecoded(instruction_partial) => instruction_partial.program_id.clone(),
                                    }
                                }
                                UiInstruction::Compiled(compiled_instruction) => {
                                    format!("Program ID Index: {}", compiled_instruction.program_id_index)
                                }
                            }
                        }).collect()
                    }
                    UiMessage::Raw(raw_message) => {
                        raw_message.instructions.iter().map(|inst| {
                            format!("Program ID Index: {}", inst.program_id_index)
                        }).collect()
                    }
                };
                
                let has_compute_budget = program_ids.iter().any(|p| p.contains("ComputeBudget"));
                let is_vote = program_ids.iter().any(|p| p.contains("Vote111111111111111111111111111111111111111"));
                let is_system = program_ids.iter().any(|p| p == "11111111111111111111111111111111");
                
                // Set landing_service based on tip status
                // If we have a tip but no known service, mark as unknown
                if !tip_recipient.is_empty() && landing_service_found.is_empty() {
                    landing_service_found = format!("Unknown: {}", tip_recipient);
                }
                // If we have no tip at all, mark as "No Tip"
                if tip_recipient.is_empty() && landing_service_found.is_empty() {
                    landing_service_found = "No Tip".to_string();
                }
                
                let fee = transaction_with_meta.meta.as_ref().map(|m| m.fee).unwrap_or(0);
                let failed = transaction_with_meta.meta.as_ref().map(|m| m.err.is_some()).unwrap_or(false);
                
                (recent_blockhash, fee, failed, has_compute_budget, is_vote, is_system, program_ids, landing_service_found, tip_recipient, tip_amount)
            }
            _ => {
                // Binary transaction - minimal data
                let fee = transaction_with_meta.meta.as_ref().map(|m| m.fee).unwrap_or(0);
                let failed = transaction_with_meta.meta.as_ref().map(|m| m.err.is_some()).unwrap_or(false);
                (String::new(), fee, failed, false, false, false, Vec::new(), "No Tip".to_string(), String::new(), 0)
            }
        };
        
        let signature = match &transaction_with_meta.transaction {
            EncodedTransaction::Json(transaction_json) => {
                if !transaction_json.signatures.is_empty() {
                    transaction_json.signatures[0].to_string()
                } else {
                    String::new()
                }
            }
            _ => String::new(),
        };
        
        transactions.push(Transaction {
            slot,
            position,
            signature,
            recent_blockhash,
            fee,
            failed: if failed { 1 } else { 0 },
            has_compute_budget: if has_compute_budget { 1 } else { 0 },
            is_vote: if is_vote { 1 } else { 0 },
            is_system: if is_system { 1 } else { 0 },
            program_ids,
            landing_service,
            tip_recipient,
            tip_amount,
        });
    }
    
    transactions
}

