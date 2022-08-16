use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::keyset_root::entry::KeysetRoot;
use deepkey_integrity::change_rule::entry::ChangeRule;

#[hdk_extern]
fn create_keyset_root((new_keyset_root, new_change_rule): (KeysetRoot, ChangeRule)) -> ExternResult<(HeaderHash, HeaderHash)> {
    Ok((create_entry(new_keyset_root)?, create_entry(new_change_rule)?))
}
