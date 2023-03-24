use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyRevocation {
    pub prior_key_registration: ActionHash,
    pub revocation_authorization: Vec<ActionHash>,
}
pub fn validate_create_key_revocation(
    _action: EntryCreationAction,
    _key_revocation: KeyRevocation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_key_revocation(
    _action: Update,
    _key_revocation: KeyRevocation,
    _original_action: EntryCreationAction,
    _original_key_revocation: KeyRevocation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_key_revocation(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_key_revocation: KeyRevocation,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Key Revocations cannot be deleted"),
        ),
    )
}
