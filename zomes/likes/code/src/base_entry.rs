// An entry against which a like is added by an agent. It could consist of a blog, post, comment, picture etc.

use hdk::holochain_core_types::time::Timeout;
use hdk::prelude::*;
use holochain_entry_utils::HolochainEntry;
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Debug, DefaultJson, Clone)]

use super::likes_entry::{}
    Like,
    LIKE_ENTRY_TYPE,
};

pub const BASE_ENTRY_TYPE: &str = 'base';

pub Type Base = String;

pub const LIKE_LINK_TAG: &str = 'liked on'
