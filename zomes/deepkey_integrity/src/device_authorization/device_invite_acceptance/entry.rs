use hdk::prelude::*;

#[cfg(test)]
use ::fixt::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DEVICE_INVITE_ACCEPTANCE_INDEX: EntryDefIndex = EntryDefIndex(2);

//#[hdk_entry(id = "device_invite_acceptance")]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInviteAcceptance {
    /// The KSRA for the invite being accepted.
    /// Not strictly required for validation as this is on the DeviceInvite.
    /// This is here as it may save network hops other than during.
    pub keyset_root_authority: ActionHash,
    pub invite: ActionHash,
}
/*
 * TODO: How do we limit to Create only?
 * 
impl TryFrom<&Record> for DeviceInviteAcceptance {
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
    DeviceInviteAcceptance;
    constructor fn new(ActionHash, ActionHash);
);

impl DeviceInviteAcceptance {
    pub fn new(keyset_root_authority: ActionHash, invite: ActionHash) -> Self {
        Self { keyset_root_authority, invite }
    }

    pub fn as_keyset_root_authority_ref(&self) -> &ActionHash {
        &self.keyset_root_authority
    }

    pub fn as_invite_ref(&self) -> &ActionHash {
        &self.invite
    }
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::DEVICE_INVITE_ACCEPTANCE_INDEX;
    use super::DeviceInviteAcceptance;

    #[test]
    fn device_invite_acceptance_index_test() {
        assert_eq!(
            DEVICE_INVITE_ACCEPTANCE_INDEX,
            entry_def_index!(DeviceInviteAcceptance).unwrap()
        )
    }
}
