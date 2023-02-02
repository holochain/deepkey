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
