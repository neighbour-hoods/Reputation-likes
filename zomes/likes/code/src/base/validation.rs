use hdk::{
    ValidationData,
    LinkValidationData,
    holochain_core_types::chain_header::ChainHeader
};

use super::entry::Base;

pub fn create(_validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn modify(_new_entry: Base,
    _old_entry: Base,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn delete(_old_entry: Base,
    _old_entry_header: ChainHeader,
    _validation_data: ValidationData) -> Result<(), String> {
    Ok(())
}

pub fn like_link(validation_data: LinkValidationData) -> Result<(), String> {
    // e-nastasia: not sure what to validate here
    match validation_data {
        hdk::LinkValidationData::LinkAdd { link: _, validation_data, } => {           
            Ok(())
        },
        hdk::LinkValidationData::LinkRemove { link: _, validation_data, } => {
            Ok(())
        },
    }
}