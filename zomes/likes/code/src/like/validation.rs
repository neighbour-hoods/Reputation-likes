use hdk::{holochain_core_types::chain_header::ChainHeader, LinkValidationData, ValidationData};

use super::entry::Like;

pub fn create(_validation_data: ValidationData) -> Result<(), String> {
    // TODO: validate that agent is only creating one like per base entry
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
