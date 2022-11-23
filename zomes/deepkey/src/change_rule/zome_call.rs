use crate::change_rule::entry::ChangeRule;
use hdk::prelude::*;

#[hdk_extern]
/// ChangeRule can be updated but not created or deleted.
/// Actually it can be created but only as part of create_keyset_root.
fn new_change_rule(
    (old_change_rule, new_change_rule): (ActionHash, ChangeRule),
) -> ExternResult<ActionHash> {
    update_entry(old_change_rule, new_change_rule)
}
