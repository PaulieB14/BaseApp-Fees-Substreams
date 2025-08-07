use substreams::log;
use hex;

#[substreams::handlers::map]
pub fn map_fee_events(block: substreams_ethereum::pb::eth::v2::Block) -> Result<Vec<u8>, substreams::errors::Error> {
    let mut events = Vec::new();
    let mut found_target_contract = false;
    
    // First pass: check if any of our target contracts are in this block
    for log in block.logs() {
        let contract_address = format!("0x{}", hex::encode(log.address()));
        if is_target_contract(&contract_address) {
            found_target_contract = true;
            break;
        }
    }
    
    // Only process if we found target contracts
    if found_target_contract {
        log::info!("Processing block {} - found target paymaster contracts", block.number);
        
        for log in block.logs() {
            let contract_address = format!("0x{}", hex::encode(log.address()));
            
            // Check if this is one of our target contracts
            if is_target_contract(&contract_address) {
                let event_signature = if !log.topics().is_empty() {
                    format!("0x{}", hex::encode(&log.topics()[0]))
                } else {
                    "0x".to_string()
                };
                
                // Check for specific fee events
                if is_fee_event(&event_signature) {
                    log::info!("Found fee event from contract: {}", contract_address);
                    log::info!("Event signature: {}", event_signature);
                    log::info!("Event topics: {:?}", log.topics().iter().map(|t| format!("0x{}", hex::encode(t))).collect::<Vec<_>>());
                    log::info!("Event data: 0x{}", hex::encode(log.data()));
                    
                    events.push(1u8); // Event counter
                }
            }
        }
    }
    
    Ok(events)
}

fn is_target_contract(address: &str) -> bool {
    address == "0x00000f79b7faf42eebadba19acc07cd08af44789" // VerifyingSingletonPaymaster
        || address == "0x00000f7365ca6c59a2c93719ad53d567ed49c14c" // BiconomyTokenPaymaster
        || address == "0x000000f6faeda8f7bfa1a8589b4b6e2d71c37592" // VerifyingSingletonPaymaster2
}

fn is_fee_event(event_signature: &str) -> bool {
    // Common fee event signatures (these are examples - you'd need the actual signatures)
    let fee_events = [
        "0x", // Placeholder - replace with actual event signatures
    ];
    
    fee_events.contains(&event_signature)
}
