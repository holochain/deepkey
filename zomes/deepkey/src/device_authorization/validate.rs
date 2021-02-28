use hdk::prelude::*;

#[hdk_extern]
fn validate_create_entry_device_authorization(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
/// Updates are not allowed for DeviceAuthorization.
fn validate_update_entry_device_authorization(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(crate::error::Error::DeviceAuthorizationUpdate.to_string())
}

#[hdk_extern]
/// Deletes are not allowed for DeviceAuthorization.
fn validate_delete_entry_device_authorization(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(crate::error::Error::DeviceAuthorizationDelete.to_string())
}