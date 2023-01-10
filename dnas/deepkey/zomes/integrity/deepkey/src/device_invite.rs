use hdi::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
// pub const DEVICE_INVITE_INDEX: EntryDefIndex = EntryDefIndex(1);

#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root: ActionHash,
    // Either the KeysetRoot or the DeviceInviteAcceptance
    pub parent: ActionHash,
    pub invitee: AgentPubKey,
}

// impl TryFrom<&Record> for DeviceInvite {
//     type Error = crate::error::Error;
//     fn try_from(element: &Element) -> Result<Self, Self::Error> {
//         match element.header() {
//             // Only creates are allowed for a DeviceInvite.
//             Header::Create(_) => {
//                 Ok(match element.entry() {
//                     ElementEntry::Present(serialized) => match Self::try_from(serialized) {
//                         Ok(deserialized) => deserialized,
//                         Err(e) => return Err(crate::error::Error::Wasm(e)),
//                     }
//                     __ => return Err(crate::error::Error::EntryMissing),
//                 })
//             },
//             _ => Err(crate::error::Error::WrongHeader),
//         }
//     }
// }

impl DeviceInvite {
    pub fn new(
        keyset_root: ActionHash,
        parent: ActionHash,
        invitee: AgentPubKey,
    ) -> Self {
        Self {
            keyset_root,
            parent,
            invitee,
        }
    }

    pub fn as_keyset_root_ref(&self) -> &ActionHash {
        &self.keyset_root
    }

    pub fn as_parent_ref(&self) -> &ActionHash {
        &self.parent
    }

    // pub fn as_device_agent_ref(&self) -> &AgentPubKey {
    //     &self.device_agent
    // }
}
