use hdk::prelude::*;
use crate::key_meta::entry::KeyMeta;
use crate::key_meta::error::Error;
use crate::validate::resolve_dependency;
use crate::key_registration::entry::KeyRegistration;
use crate::validate::ResolvedDependency;

#[hdk_extern]
fn validate_create_entry_key_meta(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let key_meta = KeyMeta::try_from(&validate_data.element)?;

    match resolve_dependency::<KeyRegistration>(key_meta.as_new_key_ref().to_owned().into())? {
        Ok(ResolvedDependency(key_registration_element, _)) => {
            if key_registration_element.header().author() != validate_data.element.header().author() {
                return Error::WrongAuthor.into();
            }
        },
        Err(validate_callback_result) => return Ok(validate_callback_result),
    }

    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_update_entry_key_meta(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
fn validate_delete_entry_key_meta(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}