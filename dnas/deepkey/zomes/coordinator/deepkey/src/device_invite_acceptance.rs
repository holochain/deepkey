use hdk::prelude::*;

use deepkey_integrity::{device_invite_acceptance::*, EntryTypes};

#[hdk_extern]
pub fn accept_invite(invite: DeviceInviteAcceptance) -> ExternResult<ActionHash> {
    let invite_acceptance_hash = create_entry(EntryTypes::DeviceInviteAcceptance(invite))?;
    Ok(invite_acceptance_hash)
}
