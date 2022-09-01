use hdk::prelude::*;
use crate::error::Error;
//use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
//use crate::keyset_root::entry::KeysetRoot;
//use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
//use crate::key_anchor::entry::KeyAnchor;
//use crate::key_registration::entry::KeyRegistration;
//use crate::change_rule::entry::ChangeRule;

pub struct ResolvedDependency<D>(pub Record, pub D);

pub fn resolve_dependency<'a, O>(hash: AnyDhtHash) -> ExternResult<Result<ResolvedDependency<O>, ValidateCallbackResult>>
    where
    O: TryFrom<SerializedBytes, Error = SerializedBytesError>
{
    let element: Record = must_get_valid_record(hash.clone().into())?;

    let output: O = match element.entry().to_app_option() {
        Ok(Some(output)) => output,
        Ok(None) => return Ok(Err(Error::EntryMissing.into())),
        Err(e) => return Ok(Err(ValidateCallbackResult::Invalid(e.to_string()))),
    };

    Ok(Ok(ResolvedDependency(element, output)))
}

/*
 * TODO: Convert to use must_get_agent_activity output instead of ValidateData
 *

fn _validate_seq_keyset(validate_data: &ValidateData, prev_element: &Record) -> ExternResult<ValidateCallbackResult> {
    // The KeysetRoot index MUST be either a KeysetRoot or a DeviceInviteAcceptance.
    if
        ( validate_data.element.header().header_seq() == KEYSET_ROOT_CHAIN_INDEX )
        && !vec![Some(&entry_type!(KeysetRoot)?), Some(&entry_type!(DeviceInviteAcceptance)?)].contains(&validate_data.element.header().entry_type()) {
            Error::KeysetSeq.into()
        }
    // The thing immediately after a KeysetRoot must be a ChangeRule
    else if (prev_element.header().entry_type() == Some(&entry_type!(KeysetRoot)?)) && (validate_data.element.header().entry_type() != Some(&entry_type!(ChangeRule)?)) {
        Error::KeysetRootThenChangeRule.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_seq_key_registration(validate_data: &ValidateData, prev_element: &Record) -> ExternResult<ValidateCallbackResult> {
    if prev_element.header().entry_type() == Some(&entry_type!(KeyRegistration)?) {
        match prev_element.header() {
            // The thing immediately after a KeyRegistration Create must be a KeyAnchor Create.
            Action::Create(_) => match validate_data.element.header() {
                Action::Create(create_header) => if create_header.entry_type == entry_type!(KeyAnchor)? {
                    Ok(ValidateCallbackResult::Valid)
                } else {
                    Error::KeyRegistrationSeq.into()
                },
                _ => Error::KeyRegistrationSeq.into(),
            },
            // The thing immediately after a KeyRegistration Update must be a KeyAnchor Update or a KeyRegistration Delete.
            Action::Update(_) => match validate_data.element.header() {
                Action::Update(update_header) => if update_header.entry_type == entry_type!(KeyAnchor)? {
                    Ok(ValidateCallbackResult::Valid)
                } else {
                    Error::KeyRegistrationSeq.into()
                },
                Action::Delete(delete_header) => {
                    match get(delete_header.deletes_address.clone(), GetOptions::content())? {
                        Some(deleted_element) => if deleted_element.header().entry_type() == Some(&entry_type!(KeyRegistration)?) {
                            Ok(ValidateCallbackResult::Valid)
                        }
                        else {
                            Error::KeyRegistrationSeq.into()
                        },
                        _ => Ok(ValidateCallbackResult::UnresolvedDependencies(vec![delete_header.deletes_address.clone().into()]))
                    }
                }
                _ => Error::KeyRegistrationSeq.into(),
            },
            // The thing immediately after a KeyRegistration Delete must be a KeyAnchor Delete.
            Action::Delete(_) => match validate_data.element.header() {
                Action::Delete(delete_header) => {
                    match get(delete_header.deletes_address.clone(), GetOptions::content())? {
                        Some(deleted_element) => if deleted_element.header().entry_type() == Some(&entry_type!(KeyAnchor)?) {
                            Ok(ValidateCallbackResult::Valid)
                        } else {
                            Error::KeyRegistrationSeq.into()
                        },
                        _ => Ok(ValidateCallbackResult::UnresolvedDependencies(vec![delete_header.deletes_address.clone().into()]))
                    }
                }
                _ => Error::KeyRegistrationSeq.into()
            }
            _ => Error::KeyRegistrationSeq.into(),
        }
    }
    // Not a KeyRegistration, don't care...
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_seq(validate_data: &ValidateData) -> ExternResult<ValidateCallbackResult> {
    match validate_data.element.header().prev_header() {
        Some(prev_header) => {
            match get(prev_header.clone(), GetOptions::content())? {
                Some(prev_element) => {
                    match _validate_seq_keyset(&validate_data, &prev_element) {
                        Ok(ValidateCallbackResult::Valid) => { },
                        validate_callback_result => return validate_callback_result,
                    }

                    _validate_seq_key_registration(&validate_data, &prev_element)
                },
                None => Ok(ValidateCallbackResult::UnresolvedDependencies(vec![prev_header.clone().into()])),
            }
        },
        // No prev header? we don't care...
        None => Ok(ValidateCallbackResult::Valid),
    }
}

 *
 */

// TODO: Old-style validate: When was this invoked?  Where do we want to call _validate_seq?
// #[hdk_extern]
// fn validate(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
//     _validate_seq(&validate_data)
// }
