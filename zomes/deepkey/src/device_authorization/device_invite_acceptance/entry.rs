use hdk::prelude::*;

#[cfg(test)]
use ::fixt::prelude::*;

#[hdk_entry(id = "device_invite_acceptance")]
#[derive(Clone)]
pub struct DeviceInviteAcceptance {
    /// The KSRA for the invite being accepted.
    /// Not strictly required for validation as this is on the DeviceInvite.
    /// This is here as it may save network hops other than during.
    pub keyset_root_authority: HeaderHash,
    invite: HeaderHash,
}

#[cfg(test)]
fixturator!(
    DeviceInviteAcceptance;
    constructor fn new(HeaderHash, HeaderHash);
);

impl DeviceInviteAcceptance {
    pub fn new(keyset_root_authority: HeaderHash, invite: HeaderHash) -> Self {
        Self { keyset_root_authority, invite }
    }

    pub fn as_keyset_root_authority_ref(&self) -> &HeaderHash {
        &self.keyset_root_authority
    }

    pub fn as_invite_ref(&self) -> &HeaderHash {
        &self.invite
    }
}