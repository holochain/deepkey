use deepkey::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn get_device_invite_acceptance(
    device_invite_acceptance_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(device_invite_acceptance_hash, GetOptions::default())
}

#[hdk_extern]
pub fn get_device_invite_acceptances_for_device_invite(
    _device_invite_hash: ActionHash,
) -> ExternResult<Vec<Record>> {
    // let links = get_links(
    //     device_invite_hash,
    //     LinkTypes::DeviceInviteToDeviceInviteAcceptances,
    //     None,
    // )?;
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
pub fn accept_invite(invite_acceptance: DeviceInviteAcceptance) -> ExternResult<ActionHash> {
    // let joining_proof = JoiningProof::new(
    //     KeysetProof::DeviceInviteAcceptance(invite_acceptance.clone()),
    //     MembraneProof::None,
    // );
    // let joining_proof_hash = create_entry(EntryTypes::JoiningProof(joining_proof))?;
    let invite_record =
        get(invite_acceptance.invite.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the invite"))
        ))?;
    let invite = invite_record
        .entry
        .to_app_option::<DeviceInvite>()
        .map_err(|e| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Could not convert entry to DeviceInvite: {:?}",
                e
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the invite"
        ))))?;

    let acceptance_hash = create_entry(EntryTypes::DeviceInviteAcceptance(
        invite_acceptance.clone(),
    ))?;

    create_link(
        invite_acceptance.keyset_root_authority.clone(),
        acceptance_hash.clone(),
        LinkTypes::KeysetRootToDeviceInviteAcceptances,
        (),
    )?;
    create_link(
        invite.invitee.clone(),
        acceptance_hash.clone(),
        LinkTypes::InviteeToDeviceInviteAcceptances,
        (),
    )?;

    Ok(acceptance_hash)
}

// Call the Deepkey Agent on another device, and send them the invitation.
#[hdk_extern]
pub fn send_device_invitation(
    (agent, dia): (AgentPubKey, DeviceInviteAcceptance),
) -> ExternResult<ZomeCallResponse> {
    let z = zome_info()?;
    let response = call_remote(agent, z.name, "receive_device_invitation".into(), None, dia)?;
    Ok(response)
}
