// Likes entry type definition

use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;

pub const LIKE_ENTRY_TYPE: &str = "like";

/* #[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
 pub struct LikeData {
    pub base: String,
    pub timestamp: i32,

}*/



#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct Like {
    base: String,
    author: Address,
    timestamp: u64,
}


impl Like {
    pub fn new(base: String, author: Address, timestamp: u64) -> Self {
        Like{
            base: base,
            timestamp: timestamp,
            author: author,
        }
    }
    pub fn entry(&self) -> Entry {
        Entry::App("like".into(), self.into())
    }
}

 

//Entry Def
pub fn like_def() ->  ValidatingEntryType {
    entry!(
        name: LIKE_ENTRY_TYPE,
        description: "A like made against a base entry in another dna or zome",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Like>| {
            Ok(())
        },
        links: [
            from!(
                &AGENT_ADDRESS,
                link_type: "link from a user to a like",
                validation_package || {
                    hdk::ValidationPackageDefinition::Entry
              },          
              validation: | _validation_data: hdk::LinkValidationData | {
                Ok(())
                }
            )
        ]
    )

}

/* passing an entry for the id of the base object that the like was sent towards
let base_entry = Entry::App(BASE_ENTRY_TYPE.into(), input_entry.base.into());
let base_address = hdk::commit_entry(&base_entry)?;
*/


/* 
   // link the like to the base entry
   hdk::link_entries(
    base: &base_Address, 
    target: &Address, 
    link_type: S, 
    tag: LIKE_LINK_TAG)
 )?;

 Ok(address)


   link the like to the user
hdk::link_entries(
    base: &AGENT_ADDRESS, 
    target: &Address, 
    link_type: S, 
    tag: LIKE_LINK_TAG)
 )?;

 Ok(address)

}

pub fn handle_get_like(address: Address) -> ZomeApiResult<Like> {
    get_as_type(address)
}
*/




