use hdk::{holochain_core_types::chain_header::ChainHeader, LinkValidationData, ValidationData};

use super::entry::Like;

pub fn create(_validation_data: ValidationData) -> Result<(), String> {
    // TODO: check that there isn't already Like for this entry with AGENT_ADDRESS in author_addr
        // 1) get all likes of this author 
        // 2) check if any of the likes have the same base entry 
        // WARNING: time complexity of this check will grow with the amount of likes a user gives.
        // Can we do something about it?
        // Kinda crazy idea: remove the timestamp field from like. Then we'd be able to do this check in O(1) time
        // by calculating hash of the resulting like and checking if it already exists
    Ok(())
}

pub fn delete(
    _old_entry: Like,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData,
) -> Result<(), String> {
    // TODO: validate that agent is only deleting their own like
    Ok(())
}

pub fn agent_link(validation_data: LinkValidationData) -> Result<(), String> {
    match validation_data {
        hdk::LinkValidationData::LinkAdd {
            link: _,
            validation_data: _,
        } => {
            // TODO: validate here that we're linking to LIke that belongs to this agent
            Ok(())
        }
        hdk::LinkValidationData::LinkRemove {
            link: _,
            validation_data: _,
        } => {
            // TODO: validate here that we're removling link to LIke that belongs to this agent
            Ok(())
        }
    }
}
