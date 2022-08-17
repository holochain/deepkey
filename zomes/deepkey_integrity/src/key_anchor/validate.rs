use hdk::prelude::*;
use crate::key_registration::entry::KeyRegistration;
use crate::key_anchor::error::Error;
use crate::key_anchor::entry::KeyAnchor;
use crate::key_registration::entry::KeyRevocation;
use crate::validate::resolve_dependency;
use crate::validate::ResolvedDependency;
use crate::key_registration::entry::KeyGeneration;
use crate::validate_classic::*;

/// The revoked key anchor must match the revoked generation of that key.
fn _validate_key_revocation(revoked_key_anchor: &KeyAnchor, key_revocation: &KeyRevocation) -> ExternResult<ValidateCallbackResult> {
    let revoked_registration: KeyRegistration = match resolve_dependency(key_revocation.as_prior_key_registration_ref().clone().into())? {
        Ok(ResolvedDependency(_, revoked_registration)) => revoked_registration,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };
    match revoked_registration {
        KeyRegistration::Create(revoked_key_generation) | KeyRegistration::Update(_, revoked_key_generation) => _validate_key_generation(revoked_key_anchor, &revoked_key_generation),
        // Cannot revoke a CreateOnly.
        KeyRegistration::CreateOnly(_) => Error::RegistrationWrongOp.into(),
        KeyRegistration::Delete(_) => Error::RegistrationWrongOp.into(),
    }
}

/// The new key anchor must match the generation of the key.
fn _validate_key_generation(proposed_key_anchor: &KeyAnchor, key_generation: &KeyGeneration) -> ExternResult<ValidateCallbackResult> {
    if key_generation.as_new_key_ref().get_raw_32() == proposed_key_anchor.as_ref() {
        Ok(ValidateCallbackResult::Valid)
    }
    else {
        Error::RegistrationWrongKey.into()
    }
}

#[hdk_extern]
/// All we care about is that the previous element created registration of this key.
fn validate_create_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_anchor = match KeyAnchor::try_from(&validate_data.element) {
        Ok(proposed_key_anchor) => proposed_key_anchor,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match validate_data.element.header().prev_header() {
        Some(prev_header) => match resolve_dependency::<KeyRegistration>(prev_header.clone().into())? {
            Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                Action::Create(_) => match key_registration {
                    KeyRegistration::Create(key_generation) | KeyRegistration::CreateOnly(key_generation) => _validate_key_generation(&proposed_key_anchor, &key_generation),
                    _ => Error::RegistrationWrongOp.into(),
                },
                _ => Error::RegistrationWrongHeader.into(),
            },
            Err(validate_callback_result) => return Ok(validate_callback_result),
        },
        None => Error::RegistrationNone.into(),
    }
}

#[hdk_extern]
/// All we care about is that the previous element updated registration of this key (but not a delete-update).
fn validate_update_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_anchor = match KeyAnchor::try_from(&validate_data.element) {
        Ok(proposed_key_anchor) => proposed_key_anchor,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match validate_data.element.header().prev_header() {
        Some(prev_header) => match resolve_dependency::<KeyRegistration>(prev_header.clone().into())? {
            Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                Action::Update(_) => match key_registration {
                    KeyRegistration::Update(key_revocation, key_generation) => {
                        match _validate_key_generation(&proposed_key_anchor, &key_generation) {
                            Ok(ValidateCallbackResult::Valid) => { },
                            validate_callback_result => return validate_callback_result,
                        }

                        match validate_data.element.header() {
                            Action::Update(key_anchor_update_header) => match resolve_dependency::<KeyAnchor>(key_anchor_update_header.original_header_address.clone().into())? {
                                Ok(ResolvedDependency(_, updated_key_anchor)) => _validate_key_revocation(&updated_key_anchor, &key_revocation),
                                Err(validate_callback_result) => Ok(validate_callback_result),
                            },
                            _ => Error::RegistrationWrongHeader.into(),
                        }
                    },
                    _ => Error::RegistrationWrongOp.into(),
                },
                _ => Error::RegistrationWrongHeader.into(),
            },
            Err(validate_callback_result) => return Ok(validate_callback_result),
        },
        None => Error::RegistrationNone.into(),
    }
}

#[hdk_extern]
/// All we care about is that the previous element deleted (revoked) the right key.
fn validate_delete_entry_key_anchor(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let prev_element: Record = match validate_data.element.header().prev_header() {
        Some(prev_header) => match get(prev_header.clone(), GetOptions::content())? {
            Some(prev_element) => prev_element,
            None => return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![prev_header.clone().into()])),
        },
        None => return Error::RegistrationNone.into(),
    };

    let revoked_key_anchor: KeyAnchor = match validate_data.element.header() {
        Action::Delete(delete_header) => {
            match resolve_dependency(delete_header.deletes_address.clone().into())? {
                Ok(ResolvedDependency(_, revoked_key_anchor)) => revoked_key_anchor,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            }
        },
        _ => return Error::RegistrationWrongHeader.into(),
    };

    match prev_element.header() {
        Action::Delete(prev_delete_header) => {
            match resolve_dependency::<KeyRegistration>(prev_delete_header.deletes_address.clone().into())? {
                Ok(ResolvedDependency(key_registration_element, key_registration)) => match key_registration_element.header() {
                    Action::Update(_) => match key_registration {
                        KeyRegistration::Delete(key_revocation) => {
                            _validate_key_revocation(&revoked_key_anchor, &key_revocation)
                        },
                        _ => Error::RegistrationWrongOp.into(),
                    },
                    _ => Error::RegistrationWrongHeader.into()
                },
                Err(validate_callback_result) => return Ok(validate_callback_result),
            }
        },
        _ => Error::RegistrationWrongHeader.into(),
    }
}
