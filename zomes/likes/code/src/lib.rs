#![feature(proc_macro_hygiene)]

use hdk::prelude::*;
use hdk_proc_macros::zome;

//Adapted from patterns developed by @pospigos in the Holo-threaded-comments DNA

mod likes_entry;
mod base_entry;

use hdk::{
    error::ZomeApiResult,
};

use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
};

use likes_entry::{
    LikeData,
    Like,
    like_def,
    handle_create_like,
    handle_get_like,
}


use base_entry::{
    base_def,
    handle_get_like,
}

define_zome! {
    entries: [
        like_def(),
        base_def().
    ]


    genesis: || { Ok(())}

    functions:  [
        create_like: {
            inputs: | like: Likesdata|,
            outputs: | result: ZomeApiResult<Address>,
            handler: handle_create_like,
        }

        get_like: {
            inputs: | address: Address|,
            outputs: | result: ZomeApiResult<Like>,
            handler: handle_get_like
        }

        get_likes: { 
            inputs: |base: String|,
            outputs: |result: ZomeApiResult<Vec<Like>>|,
            handler_ handle_get_likes, 
        }
    ]

        traits: {
            hc_public [
                create_comment,
                get_like,
                get_likes,
            ]
        }

}





















//////////////////////////////////////
/*
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]
pub struct MyEntry {
    content: String,
}

#[zome]
mod my_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        Ok(())
    }

    #[entry_def]
    fn my_entry_def() -> ValidatingEntryType {
        entry!(
            name: "my_entry",
            description: "this is a same entry defintion",
            sharing: Sharing::Public,
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: | _validation_data: hdk::EntryValidationData<MyEntry>| {
                Ok(())
            }
        )
    }

    #[zome_fn("hc_public")]
    fn create_my_entry(entry: MyEntry) -> ZomeApiResult<Address> {
        let entry = Entry::App("my_entry".into(), entry.into());
        let address = hdk::commit_entry(&entry)?;
        Ok(address)
    }

    #[zome_fn("hc_public")]
    fn get_my_entry(address: Address) -> ZomeApiResult<Option<Entry>> {
        hdk::get_entry(&address)
    }
}
*/