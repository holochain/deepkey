use deepkey_integrity::*;
use hdk::prelude::*;

use crate::source_of_authority::*;

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
// #[hdk_extern]
// pub fn get_device_invites_for_keyset_root(
//     keyset_root_hash: ActionHash,
// ) -> ExternResult<Vec<Record>> {
//     let links = get_links(keyset_root_hash, LinkTypes::KeysetRootToDeviceInvites, None)?;
//     let get_input: Vec<GetInput> = links
//         .into_iter()
//         .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
//         .collect();
//     let records: Vec<Record> = HDK
//         .with(|hdk| hdk.borrow().get(get_input))?
//         .into_iter()
//         .filter_map(|r| r)
//         .collect();
//     Ok(records)
// }
// #[hdk_extern]
// pub fn get_device_invites_for_invitee(invitee: AgentPubKey) -> ExternResult<Vec<Record>> {
//     let links = get_links(invitee, LinkTypes::InviteeToDeviceInvites, None)?;
//     let get_input: Vec<GetInput> = links
//         .into_iter()
//         .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
//         .collect();
//     let records: Vec<Record> = HDK
//         .with(|hdk| hdk.borrow().get(get_input))?
//         .into_iter()
//         .filter_map(|r| r)
//         .collect();
//     Ok(records)
// }

/// Create a new device invitation for the given agent and return the acceptance.
///
/// This function will create a new device invitation for the given agent and then return the
/// acceptance for that invitation.
///
/// This function will fail if the given agent is already a device of the current device.
#[hdk_extern]
pub fn invite_agent(agent_to_invite: AgentPubKey) -> ExternResult<DeviceInviteAcceptance> {
    // let (keyset_root, parent) = local_keyset_parent()?;

    // This is the invitor; query this chain directly using HDI
    // query(ChainQueryFilter::new()
    //     .entry_type(EntryType::App(AppEntryType::from("keyset_root".into())))
    //     .include_entries(true)
    //     .limit(1)
    // )?;
    // get_source_of_authority_action_hash((agent_info()?.agent_latest_pubkey, )))?;
    let keyset_root = query_keyset_root_action_hash(())?;
    let parent = query_keyset_authority_action_hash(())?;

    let invite = DeviceInvite::new(keyset_root.clone(), parent, agent_to_invite.clone());
    let invite_hash = create_entry(EntryTypes::DeviceInvite(invite.clone()))?;

    create_link(
        invite.keyset_root.clone(),
        invite_hash.clone(),
        LinkTypes::KeysetRootToDeviceInvites,
        (),
    )?;
    create_link(
        invite.invitee.clone(),
        invite_hash.clone(),
        LinkTypes::InviteeToDeviceInvites,
        (),
    )?;

    Ok(DeviceInviteAcceptance::new(
        keyset_root.clone(),
        invite_hash,
    ))
}
