use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::validation;
use crate::like::entry::Like;

pub const BASE_ENTRY_TO_LIKE_LINK: &str = "base_entry->like";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Base {}

impl HolochainEntry for Base {
    fn entry_type() -> String {
        String::from("base")
    }
}

pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: Base::entry_type(),
        description: "Universally unique ID of something that is being Liked",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Base>| {
            match validation_data{
                EntryValidationData::Create { validation_data, .. } => {
                    validation::create(validation_data)
                 },
                 EntryValidationData::Modify { new_entry, old_entry, old_entry_header, validation_data } => {
                    validation::modify(new_entry, old_entry, old_entry_header, validation_data)
                 },
                 EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)
                 }
            }
        },
        links: [
            to!(
                Like::entry_type(),
                link_type: BASE_ENTRY_TO_LIKE_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | validation_data: hdk::LinkValidationData| {
                    validation::like_link(validation_data)
                }
            )
        ]
    )
}
