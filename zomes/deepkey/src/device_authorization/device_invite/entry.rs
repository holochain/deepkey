use hdk::prelude::*;
#[cfg(test)]
use ::fixt::prelude::*;

#[hdk_entry(id = "device_invite")]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root_authority: HeaderHash,
    pub parent: HeaderHash,
    pub device_agent: AgentPubKey,
}

#[cfg(test)]
fixturator!(
    DeviceInvite;
    constructor fn new(HeaderHash, HeaderHash, AgentPubKey);
);

impl DeviceInvite {
    pub fn new(keyset_root_authority: HeaderHash, parent: HeaderHash, device_agent: AgentPubKey) -> Self {
        Self { keyset_root_authority, parent, device_agent }
    }

    pub fn as_keyset_root_authority_ref(&self) -> &HeaderHash {
        &self.keyset_root_authority
    }

    pub fn as_parent_ref(&self) -> &HeaderHash {
        &self.parent
    }

    pub fn as_device_agent_ref(&self) -> &AgentPubKey {
        &self.device_agent
    }
}