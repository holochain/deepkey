use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::key_meta::entry::KeyMeta;

#[hdk_extern]
fn new_key_meta(new_key_meta: KeyMeta) -> ExternResult<ActionHash> {
    create_entry(new_key_meta)
}
