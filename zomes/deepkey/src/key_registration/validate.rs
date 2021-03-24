use hdk::prelude::*;
use crate::key_registration::entry::KeyRegistration;

#[hdk_extern]
fn validate_create_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let _proposed_key_registration = match KeyRegistration::try_from(&validate_data.element) {
        Ok(key_registration) => key_registration,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    Ok(ValidateCallbackResult::Valid)
}