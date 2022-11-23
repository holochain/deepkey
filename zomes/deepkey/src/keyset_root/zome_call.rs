use deepkey_integrity::{change_rule::ChangeRule, keyset_root::KeysetRoot, *};
use hdk::prelude::*;
// use crate::change_rule::entry::ChangeRule;

#[hdk_extern]
fn create_keyset_root(
    (new_keyset_root, new_change_rule): (KeysetRoot, ChangeRule),
) -> ExternResult<(ActionHash, ActionHash)> {
    Ok((
        create_entry(EntryTypes::KeysetRoot(new_keyset_root))?,
        create_entry(EntryTypes::ChangeRule(new_change_rule))?,
    ))
}
