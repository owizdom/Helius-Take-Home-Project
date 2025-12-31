use std::collections::HashMap;

/// Known landing service tip addresses
/// These are addresses that landing services use to receive tips
pub fn get_landing_service_addresses() -> HashMap<String, String> {
    let addresses = HashMap::new();
    
    // Nozomi tip addresses (add known addresses as discovered)
    // addresses.insert("NozomiTipAddress1".to_string(), "Nozomi".to_string());
    
    // Zero Slot tip addresses
    // addresses.insert("ZeroSlotTipAddress1".to_string(), "Zero Slot".to_string());
    
    // Bloxroute tip addresses
    // addresses.insert("BloxrouteTipAddress1".to_string(), "Bloxroute".to_string());
    
    // Astralane tip addresses
    // addresses.insert("AstralaneTipAddress1".to_string(), "Astralane".to_string());
    
    // Helius tip addresses
    // addresses.insert("HeliusTipAddress1".to_string(), "Helius".to_string());
    
    addresses
}

/// Identify landing service from transfer recipient address
pub fn identify_landing_service(recipient_address: &str) -> Option<String> {
    let service_map = get_landing_service_addresses();
    service_map.get(recipient_address).cloned()
}

