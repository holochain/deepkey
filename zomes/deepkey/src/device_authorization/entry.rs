use hdk::prelude::*;

#[hdk_entry(id = "device_authorization")]
pub struct DeviceAuthorization {
    trusted_device_deepkey_agent_id_1: AgentPubKey,
    trusted_device_deepkey_agent_id_2: AgentPubKey,
    authorizor_1_sig_of_xor: Signature,
    authorizor_2_sig_of_xor: Signature,
    keyset_root_authority: EntryHash,
}

impl DeviceAuthorization {
    pub fn as_trusted_device_deepkey_agent_id_1_ref(&self) -> &AgentPubKey {
        &self.trusted_device_deepkey_agent_id_1
    }

    pub fn as_trusted_device_deepkey_agent_id_2_ref(&self) -> &AgentPubKey {
        &self.trusted_device_deepkey_agent_id_2
    }

    pub fn as_authorizor_1_sig_of_xor_ref(&self) -> &Signature {
        &self.authorizor_1_sig_of_xor
    }

    pub fn as_authorizor_2_sig_of_xor_ref(&self) -> &Signature {
        &self.authorizor_2_sig_of_xor
    }

    pub fn as_keyset_root_authority_ref(&self) -> &EntryHash {
        &self.keyset_root_authority
    }
}