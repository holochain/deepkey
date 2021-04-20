use hdk::prelude::*;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::device_authorization::device_invite::error::Error;

#[hdk_extern]
fn invite_agent(invitee: AgentPubKey) -> ExternResult<DeviceInviteAcceptance> {
    let device_invite_acceptance_query = ChainQueryFilter::new().entry_type(entry_type!(DeviceInviteAcceptance)?);
    let (keyset_root, parent) = match query(device_invite_acceptance_query)?.iter().next() {
        Some(device_invite_acceptance_element) => {
            let device_invite_acceptance = DeviceInviteAcceptance::try_from(device_invite_acceptance_element)?;
            (device_invite_acceptance.keyset_root_authority, device_invite_acceptance_element.header_hashed().as_hash().to_owned())
        },
        None => {
            let keyset_root_query = ChainQueryFilter::new().sequence_range(KEYSET_ROOT_CHAIN_INDEX..KEYSET_ROOT_CHAIN_INDEX+1);
            match query(keyset_root_query)?.iter().next() {
                Some(keyset_root_element) => {
                    let header_hash = keyset_root_element.header_hashed().as_hash();
                    (header_hash.clone(), header_hash.clone())
                },
                None => return Err(Error::MissingKeyset.into()),
            }
        }
    };
    Ok(DeviceInviteAcceptance::new(
        keyset_root.clone(),
        create_entry(DeviceInvite::new(
            keyset_root,
            parent,
            invitee,
        ))?
    ))
}