use hdi::prelude::*;


#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInviteAcceptance {
    /// The KSRA for the invite being accepted.
    /// Not strictly required for validation as this is on the DeviceInvite.
    /// This is here as it may save network hops other than during.
    pub keyset_root_authority: ActionHash,
    pub invite: ActionHash,
}

impl DeviceInviteAcceptance {
    pub fn new(keyset_root_authority: ActionHash, invite: ActionHash) -> Self {
        Self {
            keyset_root_authority,
            invite,
        }
    }
}
