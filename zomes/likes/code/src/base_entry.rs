// An entry against which a like is added by an agent. It could consist of a blog, post, comment, picture etc.

use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;

use super::likes_entry::{
    Like,
    LIKE_ENTRY_TYPE,
};

pub const BASE_ENTRY_TYPE: &str = "base";

pub type Base = String;

pub const LIKE_LINK_TAG: &str = "likedon";



pub fn base_def() -> ValidatingEntryType {
    entry!(
        name: BASE_ENTRY_TYPE,
        description: "Universally unique ID of something that is being Liked",
        sharing: Sharing::Public,
        validation_package: || {
            hdk::ValidationPackageDefinition::Entry
        },
        validation: | _validation_data: hdk::EntryValidationData<Base>| {
            Ok(())
        },
        links: [
            to!(
                LIKE_ENTRY_TYPE,
                tag:LIKE_LINK_TAG,
                validation_package: || {
                    hdk::ValidationPackageDefinition::Entry
                },
                validation: | _validation_data: hdk::LinkValidationData| {
                    Ok(())
                }
            )
        ]
    )
}

/* pub fn handle_get_likes(base: String) -> ZomeApiResult<Vec<Like>> {
    let address = hdk::entry_address(&Entry::App(BASE_ENTRY_TYPE.into(), base.into()))?;
    get_links_and_load_type(&address, LIKE_LINK_TAG)
}

*/

/*
pub fn get_links_and_load_type<R: TryFrom<AppEntryValue>>(
    base: 
)
*/
