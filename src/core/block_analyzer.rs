use clickhouse::Client;
use solana_client::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;
use crate::analyzer::types::Transaction;
use crate::analyzer::bundling::analyze_bundling;
use crate::analyzer::fee_landscape::analyze_fee_landscape;
use crate::analyzer::program_fee::analyze_program_fees;
use crate::analyzer::fee_by_type::analyze_fee_by_type;
use crate::analyzer::transaction_age::analyze_transaction_age;

async fn get_validator_key(rpc_client: &RpcClient, slot: u64, alchemy_key: &str) -> String {
    match rpc_client.get_slot_leaders(slot, 1) {
        Ok(leaders) if !leaders.is_empty() => leaders[0].to_string(),
        Err(_) | Ok(_) => {
            // Fallback: try Alchemy API
            let alchemy_url = format!("https://solana-mainnet.g.alchemy.com/v2/{}", alchemy_key);
            let alchemy_client = RpcClient::new_with_commitment(alchemy_url, CommitmentConfig::finalized());
            
            match alchemy_client.get_slot_leaders(slot, 1) {
                Ok(leaders) if !leaders.is_empty() => leaders[0].to_string(),
                Err(_) | Ok(_) => String::new(),
            }
        }
    }
}

async fn insert_rows<T: clickhouse::Row + serde::Serialize>(client: &Client, table: &str, rows: &[T]) -> Result<(), Box<dyn std::error::Error>> {
    if rows.is_empty() {
        return Ok(());
    }
    let mut inserter = client.insert(table)?;
    for row in rows {
        inserter.write(row).await?;
    }
    inserter.end().await?;
    Ok(())
}

pub async fn analyze_block(
    client: &Client,
    rpc_client: &RpcClient,
    slot: u64,
    blockhash: String,
    block_time: u64,
    transactions: &[Transaction],
    alchemy_key: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if transactions.is_empty() {
        return Err("No transactions found in block".into());
    }

    let validator_key = get_validator_key(rpc_client, slot, alchemy_key).await;

    // Bundling analysis
    let bundling_row = analyze_bundling(transactions, slot, blockhash.clone(), block_time, validator_key.clone());
    insert_rows(client, "bundling_analysis", &[bundling_row]).await?;

    // Fee landscape analysis
    let fee_row = analyze_fee_landscape(transactions, slot, block_time);
    insert_rows(client, "fee_landscape", &[fee_row]).await?;

    // Program fee analysis
    let program_fee_rows = analyze_program_fees(transactions, slot, block_time);
    insert_rows(client, "program_fee_analysis", &program_fee_rows).await?;

    // Fee by transaction type analysis
    let fee_by_type_rows = analyze_fee_by_type(transactions, slot, block_time);
    insert_rows(client, "fee_by_transaction_type", &fee_by_type_rows).await?;

    // Transaction age analysis
    let transaction_age_row = analyze_transaction_age(
        client,
        transactions,
        slot,
        block_time,
        validator_key.clone(),
    ).await?;
    insert_rows(client, "transaction_age_analysis", &[transaction_age_row]).await?;

    Ok(())
}

