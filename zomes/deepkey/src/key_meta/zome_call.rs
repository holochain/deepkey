use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::key_meta::entry::KeyMeta;
use deepkey_integrity::entry::EntryTypes;

#[hdk_extern]
fn new_key_meta(new_key_meta: KeyMeta) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::KeyMeta(new_key_meta))
}
