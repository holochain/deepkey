use hdk::prelude::*;
use crate::device_authorization::entry;

// @todo does this still make sense?
// #[hdk_extern]
// fn create_device_authorization(device: entry::DeviceAuthorization) -> ExternResult<HeaderHash> {
//     create_entry(device)
// }

#[hdk_extern]
fn invite_agent(invitee: AgentPubKey) -> ExternResult<()> {
    // @todo
    // let device_authorizatation: DeviceAuthorization = current_device_authorization();
    // create_entry(DeviceInvite {
    //     keyset_root_authority: device_authorization.keyset_root_authority(),
    //     parent: device_authorization,
    //     root_acceptance: accept(),
    //     device_agent: invitee,
    // })
    Ok(())
}