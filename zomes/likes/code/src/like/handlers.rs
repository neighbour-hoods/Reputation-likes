use crate::holochain_entry_utils::HolochainEntry;
use hdk::prelude::*;

use super::entry::Like;
use crate::base::entry::Base;

pub fn create(base: Base, author: Address, timestamp: u64) -> ZomeApiResult<Address> {
    let new_like = Like::new(base, author, timestamp);
    let like_address = hdk::commit_entry(&new_like.entry())?;
    // TODO: create link to agent here
    Ok(like_address)
}

/*
pub fn delete(base: String, author: Address, timestamp: u64) -> ZomeApiResult<Address> {

}
*/
