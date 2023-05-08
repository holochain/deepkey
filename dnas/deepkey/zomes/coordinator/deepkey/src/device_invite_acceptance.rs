use deepkey_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_device_invite_acceptance(
    device_invite_acceptance: DeviceInviteAcceptance,
) -> ExternResult<Record> {
    let device_invite_acceptance_hash = create_entry(&EntryTypes::DeviceInviteAcceptance(
        device_invite_acceptance.clone(),
    ))?;
    create_link(
        device_invite_acceptance.invite.clone(),
        device_invite_acceptance_hash.clone(),
        LinkTypes::DeviceInviteToDeviceInviteAcceptances,
        (),
    )?;
    let record = get(device_invite_acceptance_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the newly created DeviceInviteAcceptance"
        ))),
    )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_device_invite_acceptance(
    device_invite_acceptance_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(device_invite_acceptance_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_device_invite_acceptances_for_device_invite(
    device_invite_hash: ActionHash,
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
    Err(wasm_error!(WasmErrorInner::Guest(
        "Not implemented".into()
    )))
}

#[hdk_extern]
pub fn accept_invite(invite_acceptance: DeviceInviteAcceptance) -> ExternResult<ActionHash> {
    // let joining_proof = JoiningProof::new(
    //     KeysetProof::DeviceInviteAcceptance(invite_acceptance.clone()),
    //     MembraneProof::None,
    // );
    // let joining_proof_hash = create_entry(EntryTypes::JoiningProof(joining_proof))?;
    let acceptance_hash = create_entry(EntryTypes::DeviceInviteAcceptance(invite_acceptance.clone()))?;

    Ok(acceptance_hash)
}
