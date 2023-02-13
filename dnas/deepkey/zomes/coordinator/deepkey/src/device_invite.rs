use deepkey_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_device_invite(device_invite: DeviceInvite) -> ExternResult<Record> {
    let device_invite_hash = create_entry(&EntryTypes::DeviceInvite(device_invite.clone()))?;
    create_link(
        device_invite.keyset_root.clone(),
        device_invite_hash.clone(),
        LinkTypes::KeysetRootToDeviceInvites,
        (),
    )?;
    create_link(
        device_invite.invitee.clone(),
        device_invite_hash.clone(),
        LinkTypes::InviteeToDeviceInvites,
        (),
    )?;
    let record = get(device_invite_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created DeviceInvite"
        ))
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn get_device_invite(device_invite_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(device_invite_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_device_invites_for_keyset_root(
    keyset_root_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    let links = get_links(keyset_root_hash, LinkTypes::KeysetRootToDeviceInvites, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
#[hdk_extern]
pub fn get_device_invites_for_invitee(invitee: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(invitee, LinkTypes::InviteeToDeviceInvites, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}

/*
use hdk::prelude::*;

use deepkey_integrity::{
    device_invite::DeviceInvite, device_invite_acceptance::DeviceInviteAcceptance,
    keyset_root::KEYSET_ROOT_INDEX, EntryTypes, UnitEntryTypes,
};

#[hdk_extern]
/// Create a new device invitation for the given agent and return the acceptance.
///
/// This function will create a new device invitation for the given agent and then return the
/// acceptance for that invitation.
///
/// This function will fail if the given agent is already a device of the current device.
pub fn invite_agent(agent_to_invite: AgentPubKey) -> ExternResult<DeviceInviteAcceptance> {
    let (keyset_root, parent) = local_keyset_parent()?;

    let invite = DeviceInvite::new(keyset_root.clone(), parent, agent_to_invite.clone());
    let invite_header = create_entry(EntryTypes::DeviceInvite(invite.clone()))?;

    // let linkable_ksr: AnyLinkableHash = keyset_root.clone().into();
    // create_link(
    //     linkable_ksr,
    //     hash_entry(invite)?,
    //     LinkTypes::KeysetRootToDeviceInvite,
    //     (),
    // )?;

    Ok(DeviceInviteAcceptance::new(
        keyset_root.clone(),
        invite_header,
    ))
}

/// This function returns the [ActionHash] of the [KeysetRoot], and an [ActionHash] that demonstrates
/// the source of authority for this invite, either a [DeviceInviteAcceptance] or the [KeysetRoot] itself.
///
/// Searches for a [DeviceInviteAcceptance] committed to the local chain. If it finds one, it returns
/// the [ActionHash] of the [KeysetRoot] and the [ActionHash] of the [DeviceInviteAcceptance].
///
/// If it doesn't find one, then this chain is the First Deepkey Agent, so it returns the
/// [ActionHash] of the [KeysetRoot], and the [ActionHash] of the [KeysetRoot] again as the
/// self-declared source of authority.
fn local_keyset_parent() -> ExternResult<(ActionHash, ActionHash)> {
    // TODO: If this is querying for any device invite acceptance on this chain, won't it pull
    // Any of the DIA's we're generating to invite others too?
    // Should we restrict the query to only our AgentPubKey?
    let device_invite_acceptance_query = ChainQueryFilter::new()
        .entry_type(UnitEntryTypes::DeviceInviteAcceptance.try_into().unwrap());

    match query(device_invite_acceptance_query)?.into_iter().next() {
        Some(device_invite_acceptance_record) => {
            let device_invite_acceptance =
                DeviceInviteAcceptance::try_from(device_invite_acceptance_record.clone())?;
            Ok((
                device_invite_acceptance.keyset_root_authority,
                device_invite_acceptance_record.action_address().to_owned(),
            ))
        }
        None => {
            let keyset_root_query = ChainQueryFilter::new().sequence_range(
                ChainQueryFilterRange::ActionSeqRange(KEYSET_ROOT_INDEX, KEYSET_ROOT_INDEX + 1),
            );
            match query(keyset_root_query)?.into_iter().next() {
                Some(keyset_root_record) => {
                    let ksr_action_hash: ActionHash =
                        keyset_root_record.action_address().to_owned();
                    Ok((ksr_action_hash.clone(), ksr_action_hash.clone()))
                }
                None => {
                    return Err(wasm_error!(WasmErrorInner::Guest(
                        "No KeysetFound on chain".into()
                    )))
                }
            }
        }
    }
}
*/
