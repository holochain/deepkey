use hdk::prelude::*;
use crate::dna_binding::entry::DnaBinding;
use crate::dna_binding::error::Error;

#[hdk_extern]
fn validate_create_entry_dna_binding(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    DnaBinding::try_from(&validate_data.element)?;
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_update_entry_dna_binding(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
fn validate_delete_entry_dna_binding(_validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}