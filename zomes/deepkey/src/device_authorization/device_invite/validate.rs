use hdk::prelude::*;
use crate::device_authorization::device_invite::error::Error;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;

impl TryFrom<&Element> for DeviceInvite {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DeviceInvite.
            Header::Create(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match DeviceInvite::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(Error::Wasm(e)),
                    }
                    __ => return Err(Error::EntryMissing),
                })
            },
            _ => Err(Error::WrongHeader),
        }

    }
}

/// If the parent _is_ the KSRA then it gets special treatment.
/// All we care about is that the invitor is the FDA, in which case they can invite any device.
fn _validate_create_parent_ksr(create_header: &Create, parent: &KeysetRoot, _: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    // We know the parent author == parent.first_deepkey_agent as per KeysetRoot validation.
    if create_header.author != parent.first_deepkey_agent {
        Error::AuthorNotFda.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

/// If the parent references an invite, we mostly care that the ultimate KSR is a match.
/// We also care that the current invitor is the parent's invitee.
fn _validate_create_parent_device_invite_acceptance(create_header: &Create, parent_invite: &DeviceInvite, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    if parent_invite.as_keyset_root_authority_ref() != device_invite.as_keyset_root_authority_ref() {
        Error::WrongKeysetRoot.into()
    }
    else if parent_invite.as_device_agent_ref() != &create_header.author {
        Error::WrongAuthor.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

#[hdk_extern]
fn validate_create_entry_device_invite(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let device_invite = match DeviceInvite::try_from(&validate_data.element) {
        Ok(device_invite) => device_invite,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    let keyset_root_authority: KeysetRoot = match resolve_dependency(device_invite.as_keyset_root_authority_ref().to_owned().into())? {
        Ok(ResolvedDependency(_, keyset_root_authority)) => keyset_root_authority,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    if let Header::Create(create_header) = validate_data.element.header().clone() {
        // Cannot self-invite.
        // Note: A device _MAY_ still be referenced multiple times from a branching tree of invites.
        if &create_header.author == device_invite.as_device_agent_ref() {
            return Error::SelfInvite.into()
        }

        // Note that we do _NOT_ check that the `device_agent` resolves because it may not exist yet.
        // It is valid for a device to join the DHT with a reference to an invite as a joining proof.
        // The `DeviceInviteAcceptance` entry will validate the referential integrity of the `DeviceInvite`.

        if device_invite.as_keyset_root_authority_ref() == device_invite.as_parent_ref() {
            _validate_create_parent_ksr(&create_header, &keyset_root_authority, &device_invite)
        } else {
            let parent: DeviceInviteAcceptance = match resolve_dependency(device_invite.as_parent_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, device_invite_acceptance)) => device_invite_acceptance,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };
            let parent_invite: DeviceInvite = match resolve_dependency(parent.as_invite_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, device_invite)) => device_invite,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };
            _validate_create_parent_device_invite_acceptance(&create_header, &parent_invite, &device_invite)
        }
    }
    // Holochain sent the wrong header to the create callback!
    else {
        unreachable!();
    }
}

#[hdk_extern]
/// Updates are not allowed for DeviceInvite.
fn validate_update_entry_device_invite(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
/// Deletes are not allowed for DeviceInvite.
fn validate_delete_entry_device_invite(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}

#[cfg(test)]
pub mod tests {
    use crate::device_authorization::device_invite::error::Error;
    use fixt::prelude::*;
    use holochain_types::prelude::ValidateDataFixturator;

    #[test]
    fn test_validate_update_entry_device_invite() {
        assert_eq!(
            super::validate_update_entry_device_invite(fixt!(ValidateData)),
            Error::UpdateAttempted.into(),
        );
    }

    #[test]
    fn test_validate_delete_entry_device_invite() {
        assert_eq!(
            super::validate_delete_entry_device_invite(fixt!(ValidateData)),
            Error::DeleteAttempted.into(),
        );
    }
}