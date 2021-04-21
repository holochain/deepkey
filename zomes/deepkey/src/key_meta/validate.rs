use hdk::prelude::*;
use crate::key_meta::entry::KeyMeta;
use crate::key_meta::error::Error;

#[hdk_extern]
fn validate_create_entry_key_meta(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    KeyMeta::try_from(&validate_data.element)?;
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