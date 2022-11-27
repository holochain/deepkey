use hdk::prelude::*;
use crate::key_meta::entry::KeyMeta;

#[hdk_extern]
fn new_key_meta(new_key_meta: KeyMeta) -> ExternResult<HeaderHash> {
    create_entry(new_key_meta)
}