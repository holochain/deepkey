use hdk::prelude::*;

pub type Acceptance = (AgentPubKey, Signature);

#[hdk_entry(id = "device_invite")]
pub struct DeviceInvite {
    keyset_root_authority: HeaderHash,
    parent: HeaderHash,
    root_acceptance: Acceptance,
    device_agent: AgentPubKey,
}