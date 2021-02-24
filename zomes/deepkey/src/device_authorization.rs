use hdk::prelude::*;

#[hdk_entry(id = "device_authorization")]
pub struct DeviceAuthorization {
    trusted_device_deepkey_agent_id_1: AgentPubKey,
    trusted_device_deepkey_agent_id_2: AgentPubKey,
    authorizor_1_sig_of_xor: Signature,
    authorizor_2_sig_of_xor: Signature,
    keyset_root_authority: EntryHash,
}

// impl DeviceAuthorization {
//     pub fn xor
// }

#[hdk_extern]
fn create_device_authorization(device: DeviceAuthorization) -> ExternResult<HeaderHash> {
    create_entry(device)
}