use crate::holochain_entry_utils::HolochainEntry;
use hdk::{prelude::*, AGENT_ADDRESS};

use super::entry::{Like, LIKE_FROM_AGENT_LINK};
use crate::base::entry::BASE_ENTRY_TO_LIKE_LINK;

pub fn create(base_addr: Address, author_addr: Address, timestamp: u64) -> ZomeApiResult<Address> {
    let new_like = Like::new(base_addr.clone(), author_addr, timestamp);
    let like_address = hdk::commit_entry(&new_like.entry())?;
    hdk::link_entries(&like_address, &AGENT_ADDRESS, LIKE_FROM_AGENT_LINK, "")?;
    hdk::link_entries(&base_addr, &like_address, BASE_ENTRY_TO_LIKE_LINK, "")?;
    Ok(like_address)
}

pub fn delete(address: Address) -> ZomeApiResult<Address> {
    // (e-nastasia) Q: do we have to remove the links for like entry or would they be marked as deleted automatically?
    // asked about it here: https://forum.holochain.org/t/what-happens-with-links-to-from-removed-entries/3462
    let like: Like = hdk::utils::get_as_type(address.clone())?;
    // (e-nastasia) NOTE: like could be cascade deleted by the agent who isn't it's author when it's base entry
    //  is removed in another DNA. To avoid imposing restrictions on likes DNA usage, we can't require
    //  this operation to be performed only by base entry author (because this another DNA could have some other rules).
    //  So we're assuming that this delete operation is already validated by like validation rules
    //  and we don't have to perform any additional checks.
    hdk::remove_link(&address, &like.author_addr, LIKE_FROM_AGENT_LINK, "")?;
    hdk::remove_link(
        &like.base_addr,
        &like.address()?.clone(),
        BASE_ENTRY_TO_LIKE_LINK,
        "",
    )?;
    hdk::remove_entry(&address)
}
