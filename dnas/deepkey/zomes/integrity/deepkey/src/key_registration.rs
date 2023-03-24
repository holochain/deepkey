use hdi::prelude::*;

use crate::{KeyGeneration, KeyRevocation};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub enum KeyRegistration {
    Create(KeyGeneration), // Creates a key under management of current KSR on this chain
    CreateOnly(KeyGeneration), // Keys for hosted web users may be of this type, cannot replace/revoke
    Update(KeyRevocation, KeyGeneration), // revokes a key and replaces it with a newly generated one
    Delete(KeyRevocation) // permanently revokes a key (Note: still uses an update action.)
}
pub fn validate_create_key_registration(
    _action: EntryCreationAction,
    _key_registration: KeyRegistration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_key_registration(
    _action: Update,
    _key_registration: KeyRegistration,
    _original_action: EntryCreationAction,
    _original_key_registration: KeyRegistration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_key_registration(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_key_registration: KeyRegistration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
