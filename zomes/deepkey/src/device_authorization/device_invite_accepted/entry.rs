use hdk::prelude::*;
use crate::device_authorization::device_invite::entry::Acceptance;

#[hdk_entry(id = "device_invite_accepted")]
pub struct DeviceInviteAccepted {
    invite: HeaderHash,
    device_acceptance: Acceptance,
}