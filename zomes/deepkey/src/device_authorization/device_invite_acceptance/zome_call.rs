use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use deepkey_integrity::device_authorization::device_invite::entry::DeviceInvite;
use deepkey_integrity::device_authorization::inbox::DEVICE_INVITE_LINK_TAG_BYTES;
use deepkey_integrity::entry::{ EntryTypes, LinkTypes };

use crate::device_authorization::device_invite::local_keyset_parent;

#[hdk_extern]
fn accept_invite(device_invite: DeviceInvite) -> ExternResult<Option<ActionHash>> {
    match get(hash_entry(device_invite.clone())?, GetOptions::latest())? {
        Some(invite_element) => Ok(Some(confirm_acceptance(DeviceInviteAcceptance::new(
            device_invite.keyset_root_authority,
            invite_element.action_hashed().as_hash().to_owned()
        ))?)),
        None => Ok(None)
    }
}

#[hdk_extern]
/// "Best effort" attempt to ignore an invite.
/// Really just attempts to get and delete the link from the inbox.
fn ignore_invite(device_invite: DeviceInvite) -> ExternResult<()> {
    let links: Vec<Link> = get_links(
        agent_info()?.agent_latest_pubkey,
	LinkTypes::AgentInviteNotify,
        Some(LinkTag(DEVICE_INVITE_LINK_TAG_BYTES.to_vec()))
    )?;

    let invite_hash = hash_entry(device_invite)?;

    for link in links.into_iter() {
        if link.target == invite_hash.clone().into() {
            delete_link(link.create_link_hash)?;
        }
    }

    Ok(())
}

#[hdk_extern]
fn confirm_acceptance(device_invite_acceptance: DeviceInviteAcceptance) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::DeviceInviteAcceptance(device_invite_acceptance))
}

#[hdk_extern]
fn check_inbox(_: ()) -> ExternResult<Vec<DeviceInvite>> {
    let links: Vec<Link> = get_links(
        agent_info()?.agent_latest_pubkey,
	LinkTypes::AgentInviteNotify,
        Some(LinkTag(DEVICE_INVITE_LINK_TAG_BYTES.to_vec()))
    )?;

    let (keyset_root, _) = local_keyset_parent()?;

    let mut invites: Vec<DeviceInvite> = Vec::new();
    for link in links.iter() {
        match get(ActionHash::from_raw_39(
            link.tag.clone().into_inner()[DEVICE_INVITE_LINK_TAG_BYTES.len()..].to_vec()
        ).map_err(|e| wasm_error!(WasmErrorInner::Guest(e.to_string())))?, GetOptions::latest())? {
            Some(invite_element) => {
                let invite = DeviceInvite::try_from(&invite_element)?;

                // Silently ignore and cleanup any invites to the current keyset.
                if invite.keyset_root_authority == keyset_root {
                    delete_link(link.create_link_hash.clone())?;
                }
                else {
                    invites.push(DeviceInvite::try_from(&invite_element)?)
                }
            }
            None => { },
        }
    }
    Ok(invites)
}
