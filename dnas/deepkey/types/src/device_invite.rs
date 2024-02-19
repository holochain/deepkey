use hdi::prelude::*;

use crate::*;


#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root: ActionHash,
    // Either the KeysetRoot or the DeviceInviteAcceptance
    pub parent: ActionHash,
    pub invitee: AgentPubKey,
}

impl DeviceInvite {
    pub fn new(keyset_root: ActionHash, parent: ActionHash, invitee: AgentPubKey) -> Self {
        Self {
            keyset_root,
            parent,
            invitee,
        }
    }
}
