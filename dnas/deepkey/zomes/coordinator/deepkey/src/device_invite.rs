use deepkey_integrity::*;
use hdk::prelude::*;

use crate::source_of_authority::*;

#[hdk_extern]
pub fn get_device_invite(device_invite_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(device_invite_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_device_invites_for_keyset_root(
    _keyset_root_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    // let links = get_links(keyset_root_hash, LinkTypes::KeysetRootToDeviceInvites, None)?;
    // let get_input: Vec<GetInput> = links
    //     .into_iter()
    //     .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
    //     .collect();
    // let records: Vec<Record> = HDK
    //     .with(|hdk| hdk.borrow().get(get_input))?
    //     .into_iter()
    //     .filter_map(|r| r)
    //     .collect();
    // Ok(records)
    Err(wasm_error!(WasmErrorInner::Guest("Not implemented".into())))
}
#[hdk_extern]
pub fn get_device_invites_for_invitee(_invitee: AgentPubKey) -> ExternResult<Vec<Record>> {
    // let links = get_links(invitee, LinkTypes::InviteeToDeviceInvites, None)?;
    // let get_input: Vec<GetInput> = links
    //     .into_iter()
    //     .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
    //     .collect();
    // let records: Vec<Record> = HDK
    //     .with(|hdk| hdk.borrow().get(get_input))?
    //     .into_iter()
    //     .filter_map(|r| r)
    //     .collect();
    // Ok(records)
    Err(wasm_error!(WasmErrorInner::Guest("Not implemented".into())))
}

/// Create a new device invitation for the given agent and return the acceptance.
///
/// This function will create a new device invitation for the given agent and then return the
/// acceptance for that invitation.
///
/// This function will fail if the given agent is already a device of the current device.
#[hdk_extern]
pub fn invite_agent(agent_to_invite: AgentPubKey) -> ExternResult<DeviceInviteAcceptance> {
    // This is the invitor; query this chain directly using HDI
    // query(ChainQueryFilter::new()
    //     .entry_type(EntryType::App(AppEntryType::from("keyset_root".into())))
    //     .include_entries(true)
    //     .limit(1)
    // )?;
    let keyset_root = query_keyset_root_action_hash(())?;
    debug!("Using keyset root: {}", keyset_root );
    let parent = query_keyset_authority_action_hash(())?;
    debug!("Using parent: {}", parent );

    let invite = DeviceInvite::new(keyset_root.clone(), parent, agent_to_invite.clone());
    debug!("Constructed invite: {:#?}", invite );
    let invite_hash = create_entry(EntryTypes::DeviceInvite(invite.clone()))?;
    debug!("Created invite: {}", invite_hash );

    let acceptance = DeviceInviteAcceptance::new(
        keyset_root.clone(),
        invite_hash,
    );
    debug!("Return invite acceptance: {:#?}", acceptance );
    Ok( acceptance )
}
