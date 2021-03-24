use hdk::prelude::*;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_acceptance::error::Error;

impl TryFrom<&Element> for DeviceInviteAcceptance {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DeviceInvite.
            Header::Create(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match DeviceInviteAcceptance::try_from(serialized) {
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

fn _validate_create_authorization(create_header: &Create, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    // Only the intended recipient of a device invite can accept it.
    if &create_header.author != device_invite.as_device_agent_ref() {
        Error::WrongAuthor.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_create_ksr(device_invite: &DeviceInvite, device_invite_acceptance: &DeviceInviteAcceptance) -> ExternResult<ValidateCallbackResult> {
    if device_invite.as_keyset_root_authority_ref() != device_invite_acceptance.as_keyset_root_authority_ref() {
        Error::WrongKeysetRoot.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

#[hdk_extern]
fn validate_create_entry_device_invite_acceptance(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let device_invite_acceptance = match DeviceInviteAcceptance::try_from(&validate_data.element) {
        Ok(device_invite_acceptance) => device_invite_acceptance,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    let device_invite = match resolve_dependency(device_invite_acceptance.as_invite_ref().to_owned().into())? {
        Ok(ResolvedDependency(_, device_invite)) => device_invite,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    if let Header::Create(create_header) = validate_data.element.header().clone() {
        match _validate_create_authorization(&create_header, &device_invite) {
            Ok(ValidateCallbackResult::Valid) => { },
            validate_callback_result => return validate_callback_result,
        }

        _validate_create_ksr(&device_invite, &device_invite_acceptance)
    }
    else {
        unreachable!();
    }
}

#[hdk_extern]
fn validate_update_entry_device_invite_acceptance(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
fn validate_delete_entry_device_invite_acceptance(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use holochain_types::prelude::*;
    use crate::device_authorization::device_invite_acceptance::error::Error;
    use ::fixt::prelude::*;
    use holochain_types::prelude::ValidateDataFixturator;
    use crate::device_authorization::device_invite::entry::DeviceInviteFixturator;
    use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptanceFixturator;

    #[test]
    fn test_validate_update_entry_device_invite_acceptance() {
        assert_eq!(
            super::validate_update_entry_device_invite_acceptance(fixt!(ValidateData)),
            Error::UpdateAttempted.into(),
        );
    }

    #[test]
    fn test_validate_delete_entry_device_invite_acceptance() {
        assert_eq!(
            super::validate_delete_entry_device_invite_acceptance(fixt!(ValidateData)),
            Error::DeleteAttempted.into(),
        );
    }

    #[test]
    fn test_validate_create_authorization() {
        let mut create_header = fixt!(Create);
        let device_invite = fixt!(DeviceInvite);

        assert_eq!(
            super::_validate_create_authorization(&create_header, &device_invite),
            Error::WrongAuthor.into(),
        );

        create_header.author = device_invite.as_device_agent_ref().clone();

        assert_eq!(
            super::_validate_create_authorization(&create_header, &device_invite),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_ksr() {
        let device_invite = fixt!(DeviceInvite);
        let mut device_invite_acceptance = fixt!(DeviceInviteAcceptance);

        assert_eq!(
            super::_validate_create_ksr(&device_invite, &device_invite_acceptance),
            Error::WrongKeysetRoot.into(),
        );

        device_invite_acceptance.keyset_root_authority = device_invite.as_keyset_root_authority_ref().clone();

        assert_eq!(
            super::_validate_create_ksr(&device_invite, &device_invite_acceptance),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_entry_device_invite_acceptance() {
        let mut validate_data = fixt!(ValidateData);

        let mut create_header = fixt!(Create);
        let mut device_invite_acceptance = fixt!(DeviceInviteAcceptance);

        let device_invite = fixt!(DeviceInvite);
        let device_invite_create_header = fixt!(Create);
        let mut device_invite_element = fixt!(Element);
        *device_invite_element.as_header_mut() = Header::Create(device_invite_create_header);
        *device_invite_element.as_entry_mut() = ElementEntry::Present(device_invite.clone().try_into().unwrap());

        create_header.author = device_invite.as_device_agent_ref().clone();
        device_invite_acceptance.keyset_root_authority = device_invite.as_keyset_root_authority_ref().clone();
        *validate_data.element.as_header_mut() = Header::Create(create_header);

        assert_eq!(
            super::validate_create_entry_device_invite_acceptance(validate_data.clone()),
            Error::EntryMissing.into(),
        );

        *validate_data.element.as_entry_mut() = ElementEntry::Present(device_invite_acceptance.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    device_invite_acceptance.as_invite_ref().to_owned().into(),
                    GetOptions::content(),
                )
            ))
            .times(1)
            .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite_acceptance(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_invite_acceptance.as_invite_ref().to_owned().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    device_invite_acceptance.as_invite_ref().to_owned().into(),
                    GetOptions::content(),
                )
            ))
            .times(1)
            .return_const(Ok(Some(device_invite_element)));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite_acceptance(validate_data.clone()),
            Ok(ValidateCallbackResult::Valid),
        );
    }
}