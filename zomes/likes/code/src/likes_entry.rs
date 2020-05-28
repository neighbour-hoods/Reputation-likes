// Likes entry type definition



use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]

pub const LIKE_ENTRY_TYPE: &str = "like";

pub struct LikeData {
    pub base: String
    pub timestamp: i32

}

pub struct Like {
    base: String
    author: Address
    timestamp: i32
}

// Convert and input Like, which is without an author into a Like entry

impl LikeData {
    pub fn with_author(&self, author: Address,) -> Comment {
        Comment{
            base: self.base.clone()
            timestamp: self.base.clone()
            author,
        }
    }
}

pub fn handle_create_like(input_entry: Likedata) -> ZomeApiResult<Address> {
    let entry = Entry::App(
        input_entry.with_author(
            AGENT_ADDRESS.to_string().into()
        ).into()
    );
    let address = hdk::commit_entry(&entry)?;
    
 // passing an entry for the id of the base object that the like was sent towards
    let base_entry = Entry::App(BASE_ENTRY_TYPE.into(), input_entry.base.into());
    let base_address = hdk::commit_entry(&base_entry)?;

    // link the like to the base entry

    hdk::link_entries(
    &base_address,
    &address
    link type?
    LIKE_LINK_TAG,
    )?;

    Ok(address)

}

pub fn handle_get_like(address: Address) -> ZomeApiResult<Like> {
    get_as_type(address)
}

//Entry Def

pub fn like_def() ->  {
    entry!(
        name: LIKE_ENTRY_TYPE,
        description: "A like made against a base entry in another dna or zome",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        
        },
        validation: | _validation_data: hdk::EntryValidationData<Comment>| {
            Ok(())
        }
    )
}