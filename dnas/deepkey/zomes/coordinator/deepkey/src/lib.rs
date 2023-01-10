pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod keyset_root;

use hdk::prelude::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    Ok(InitCallbackResult::Pass)
}
