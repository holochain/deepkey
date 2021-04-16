use hdk::prelude::*;
use crate::device_authorization::device_invite_acceptance::error::Error;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::device_authorization::device_invite::entry::DeviceInvite;

#[hdk_extern]
fn accept_invite(invite_header_hash: HeaderHash) -> ExternResult<HeaderHash> {
    match get(invite_header_hash.clone(), GetOptions::content())? {
        Some(invite_element) => {
            let invite = DeviceInvite::try_from(&invite_element)?;
            create_entry(DeviceInviteAcceptance::new(
                invite.keyset_root_authority,
                invite_header_hash,
            ))
        },
        None => Err(Error::InviteNotFound.into()),
    }
}