use hdk::prelude::*;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;

#[hdk_extern]
fn accept_invite(device_invite_acceptance: DeviceInviteAcceptance) -> ExternResult<HeaderHash> {
    create_entry(device_invite_acceptance)
}