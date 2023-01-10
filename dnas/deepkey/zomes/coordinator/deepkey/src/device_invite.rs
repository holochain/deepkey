use hdk::prelude::*;

use deepkey_integrity::{
    device_invite::DeviceInvite, device_invite_acceptance::DeviceInviteAcceptance,
    keyset_root::KEYSET_ROOT_CHAIN_INDEX, EntryTypes, LinkTypes, UnitEntryTypes,
};

#[hdk_extern]
pub fn invite_agent(agent_to_invite: AgentPubKey) -> ExternResult<DeviceInviteAcceptance> {
    let (keyset_root, parent) = local_keyset_parent()?;

    let invite = DeviceInvite::new(keyset_root.clone(), parent, agent_to_invite.clone());
    let invite_header = create_entry(EntryTypes::DeviceInvite(invite.clone()))?;

    let linkable_ksr: AnyLinkableHash = keyset_root.clone().into();
    create_link(
        linkable_ksr,
        hash_entry(invite)?,
        LinkTypes::AgentPubKeyToDeviceInvite,
        (),
    )?;

    Ok(DeviceInviteAcceptance::new(
        keyset_root.clone(),
        invite_header,
    ))
}

// Parent: An `ActionHash` referring to the invitor's direct parent in the keyset tree, 
// which is either its KSR 
// or its current `DeviceInviteAcceptance`. 
// This is used to establish the chain of authority from the original KSR.
pub fn local_keyset_parent() -> ExternResult<(ActionHash, ActionHash)> {
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
            let keyset_root_query =
                ChainQueryFilter::new().sequence_range(ChainQueryFilterRange::ActionSeqRange(
                    KEYSET_ROOT_CHAIN_INDEX,
                    KEYSET_ROOT_CHAIN_INDEX + 1,
                ));
            match query(keyset_root_query)?.into_iter().next() {
                Some(keyset_root_record) => {
                    let ksr_action_hash: ActionHash = keyset_root_record.action_address().to_owned();
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
