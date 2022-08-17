use hdk::prelude::*;
#[cfg(test)]
use ::fixt::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DEVICE_INVITE_INDEX: EntryDefIndex = EntryDefIndex(1);

//#[hdk_entry(id = "device_invite", required_validation_type = "full")]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root_authority: ActionHash,
    // Either the KeysetRoot or the DeviceInviteAcceptance
    pub parent: ActionHash,
    pub device_agent: AgentPubKey,
}
/*
 * TODO: How do we limit to Create only?
 * 
impl TryFrom<&Record> for DeviceInvite {
    type Error = crate::error::Error;
    fn try_from(element: &Record) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DeviceInvite.
            Action::Create(_) => {
                Ok(match element.entry() {
                    RecordEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}
 */
#[cfg(test)]
fixturator!(
    DeviceInvite;
    constructor fn new(ActionHash, ActionHash, AgentPubKey);
);

impl DeviceInvite {
    pub fn new(keyset_root_authority: ActionHash, parent: ActionHash, device_agent: AgentPubKey) -> Self {
        Self { keyset_root_authority, parent, device_agent }
    }

    pub fn as_keyset_root_authority_ref(&self) -> &ActionHash {
        &self.keyset_root_authority
    }

    pub fn as_parent_ref(&self) -> &ActionHash {
        &self.parent
    }

    pub fn as_device_agent_ref(&self) -> &AgentPubKey {
        &self.device_agent
    }
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::DEVICE_INVITE_INDEX;
    use super::DeviceInvite;

    #[test]
    fn device_invite_index_test() {
        assert_eq!(
            DEVICE_INVITE_INDEX,
            entry_def_index!(DeviceInvite).unwrap()
        )
    }
}
