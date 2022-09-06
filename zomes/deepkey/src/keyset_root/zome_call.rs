use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::keyset_root::entry::KeysetRoot;
use deepkey_integrity::change_rule::entry::ChangeRule;
use deepkey_integrity::entry::EntryTypes;

#[hdk_extern]
fn create_keyset_root((new_keyset_root, new_change_rule): (KeysetRoot, ChangeRule)) -> ExternResult<(ActionHash, ActionHash)> {
    Ok((create_entry(EntryTypes::KeysetRoot(new_keyset_root))?,
        create_entry(EntryTypes::ChangeRule(new_change_rule))?))
}
