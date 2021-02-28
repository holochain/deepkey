use hdk::prelude::*;

pub type Acceptance = (AgentPubKey, Signature);

#[hdk_entry(id = "device_authorization")]
pub struct DeviceAuthorization {
    /// Each DeviceAuthorization comes under a singe KeysetRoot as the 'authority'
    keyset_root_authority: HeaderHash,
    /// DeviceAuthorization entries form a tree up to a KeysetRoot on the `root_acceptance` side.
    /// If the parent references the `AgentPubKey` in the child's `root_acceptance` then we know by induction
    /// that the root agent in this entry comes under the `keyset_root_authority`.
    /// At the top of the tree the parent will be the KeysetRoot in which case the DA
    /// on the KSR chain must have a `root_acceptance` agent that is the FDA and the DA/KSR authors match.
    parent: HeaderHash,
    root_acceptance: Acceptance,
    device_acceptance: Acceptance,
}

impl DeviceAuthorization {
    pub fn validate_signatures(&self) -> ExternResult<bool> {
        let mut bytes = [self.root_acceptance.0.get_raw_32(), self.device_acceptance.0.get_raw_32()].concat();
        Ok(
            verify_signature_raw(self.root_acceptance.0.clone(), self.root_acceptance.1.clone(), bytes.to_vec())?
            && verify_signature_raw(self.device_acceptance.0.clone(), self.device_acceptance.1.clone(), bytes.to_vec())?
        )
    }

    pub fn as_keyset_root_authority_ref(&self) -> &HeaderHash {
        &self.keyset_root_authority
    }

    pub fn as_parent_ref(&self) -> &HeaderHash {
        &self.parent
    }

    pub fn as_root_acceptance_ref(&self) -> &Acceptance {
        &self.root_acceptance
    }

    pub fn as_device_acceptance_ref(&self) -> &Acceptance {
        &self.device_acceptance
    }
}