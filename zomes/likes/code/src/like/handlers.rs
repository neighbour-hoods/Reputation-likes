use super::entry::Like;

pub fn create(base: String, author: Address, timestamp: u64) -> ZomeApiResult<Address> {
    let new_like = Like::new(base, author, timestamp);
    let like_address = hdk::commit_entry(&new_like.entry())?;
    // TODO: create link to agent here
    Ok(like_address)
}

/*
pub fn delete(base: String, author: Address, timestamp: u64) -> ZomeApiResult<Address> {

}
*/