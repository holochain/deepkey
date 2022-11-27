use hdk::prelude::*;
#[cfg(test)]
use ::fixt::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DEVICE_INVITE_INDEX: EntryDefIndex = EntryDefIndex(1);

#[hdk_entry(id = "device_invite", required_validation_type = "full")]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root_authority: HeaderHash,
    // Either the KeysetRoot or the DeviceInviteAcceptance
    pub parent: HeaderHash,
    pub device_agent: AgentPubKey,
}

impl TryFrom<&Element> for DeviceInvite {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DeviceInvite.
            Header::Create(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match Self::try_from(serialized) {
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