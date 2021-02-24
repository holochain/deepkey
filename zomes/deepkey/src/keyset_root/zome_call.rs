use hdk::prelude::*;
use crate::keyset_root::entry;

#[hdk_extern]
fn create_keyset_root(new_keyset_root: entry::KeysetRoot) -> ExternResult<HeaderHash> {
    create_entry(new_keyset_root)
}