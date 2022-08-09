use hdk::prelude::*;
use crate::change_rule::entry::ChangeRule;

#[hdk_extern]
/// ChangeRule can be updated but not created or deleted.
/// Actually it can be created but only as part of create_keyset_root.
fn new_change_rule((old_change_rule, new_change_rule): (HeaderHash, ChangeRule)) -> ExternResult<HeaderHash> {
    update_entry(old_change_rule, new_change_rule)
}