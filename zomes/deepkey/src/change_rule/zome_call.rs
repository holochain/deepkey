use hdk::prelude::*;
use crate::change_rule::entry::ChangeRule;

#[hdk_extern]
fn create_key_change_rule(new_change_rule: ChangeRule) -> ExternResult<HeaderHash> {
    create_entry(new_change_rule)
}

#[hdk_extern]
fn update_key_change_rule((old_change_rule, new_change_rule): (HeaderHash, ChangeRule)) -> ExternResult<HeaderHash> {
    update_entry(old_change_rule, new_change_rule)
}