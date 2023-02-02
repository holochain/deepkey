pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod keyset_root;

use hdk::prelude::*;
use keyset_root::create_keyset_root;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // TODO: Check for joining proof; if it's a device invite acceptance, initial joiningProof should
    // include this instead of creating a KSR.
    create_keyset_root()?;
    Ok(InitCallbackResult::Pass)
}
