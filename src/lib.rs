use substreams::log;

#[substreams::handlers::map]
pub fn map_fee_events(block: substreams_ethereum::pb::eth::v2::Block) -> Result<Vec<u8>, substreams::errors::Error> {
    let mut events = Vec::new();
    
    for log in block.logs() {
        let contract_address = format!("0x{}", hex::encode(log.address()));
        
        // Check if this is one of our target contracts
        if is_target_contract(&contract_address) {
            log::info!("Found event from contract: {}", contract_address);
            
            // Log event details for debugging
            log::info!("Event topics: {:?}", log.topics().iter().map(|t| format!("0x{}", hex::encode(t))).collect::<Vec<_>>());
            log::info!("Event data: 0x{}", hex::encode(log.data()));
            
            events.push(1u8); // Simple event counter
        }
    }
    
    Ok(events)
}

fn is_target_contract(address: &str) -> bool {
    address == "0x00000f79b7faf42eebadba19acc07cd08af44789" // VerifyingSingletonPaymaster
        || address == "0x00000f7365ca6c59a2c93719ad53d567ed49c14c" // BiconomyTokenPaymaster
        || address == "0x000000f6faeda8f7bfa1a8589b4b6e2d71c37592" // VerifyingSingletonPaymaster2
}
