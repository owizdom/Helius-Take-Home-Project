use std::collections::HashMap;

pub fn get_landing_service_addresses() -> HashMap<String, String> {
    let mut addresses = HashMap::new();
    
    // Jito landing service and tip accounts
    // These are all Jito tip accounts that route through the same landing service
    let jito_addresses = vec![
        "ADuUkR4vqLUMWXxW9gh6D6L8pMSawimctcNZ5pGwDcEt", // Jito landing service
        "87wyLh2iDzszjYTPi5tnDhRx5GGrxzWsRAUbBboVm743", // Jito tip account
        "ADaUMid9yfUytqMBgopwjb2DTLSokTSzL1zt6iGPaS49", // Jito tip account
        "Cw8CFyM9FkoMi7K7Crf6HNQqf4uEMzpKw6QNghXLvLkY", // Jito tip account
        "HFqU5x63VTqvQss8hp11i4wVV8bD44PvwucfZ2bU7gRe", // Jito tip account
        "DfXygSm4jCyNCybVYYK6DwvWqjKee8pbDmJGcLWNDXjh", // Jito tip account
        "96gYZGLnJYVFmbjzopPSU6QiEV5fGqZNyN9nmNhvrZU5", // Jito tip account
        "3AVi9Tg9Uo68tJfuvoKvqKNWKkC5wPdSSdeBnizKZ6jT", // Jito tip account
        "DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL", // Jito tip account
    ];
    
    for addr in jito_addresses {
        addresses.insert(addr.to_string(), "Jito".to_string());
    }
    
    addresses
}

/// Identify landing service from transfer recipient address
pub fn identify_landing_service(recipient_address: &str) -> Option<String> {
    let service_map = get_landing_service_addresses();
    service_map.get(recipient_address).cloned()
}

