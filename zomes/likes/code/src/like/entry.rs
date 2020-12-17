use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;

use super::validation;

pub const LIKE_FROM_AGENT_LINK: &str = "agent->like";

#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Like {
    pub base_addr: Address,
    pub author_addr: Address,
    pub timestamp: u64,
}

// e-nastasia: implementing HolochainEntry gives access to several helper methods
// so we don't have to implement them ourselves
impl HolochainEntry for Like {
    fn entry_type() -> String {
        String::from("like")
    }
}

impl Like {
    pub fn new(base_addr: Address, author_addr: Address, timestamp: u64) -> Self {
        Like {
            base_addr: base_addr,
            timestamp: timestamp,
            author_addr: author_addr,
        }
    }
}

pub fn like_def() -> ValidatingEntryType {
    entry!(
        name: Like::entry_type(),
        description: "A like made against a base entry in another dna or zome",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | validation_data: hdk::EntryValidationData<Like>| {
            match validation_data{
                EntryValidationData::Create { validation_data, .. } => {
                    validation::create(validation_data)
                 },
                 EntryValidationData::Modify { .. } => {
                    return Err(String::from("Cannot modify like: only create & delete are allowed"));
                 },
                 EntryValidationData::Delete { old_entry, old_entry_header, validation_data } => {
                    validation::delete(old_entry, old_entry_header, validation_data)
                 }
            }
        },
        links: [
            from!(
                "%agent_id",
                link_type: LIKE_FROM_AGENT_LINK,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
              },
              validation: | validation_data: hdk::LinkValidationData | {
                  validation::agent_link(validation_data)
                }
            )
        ]
    )
}
