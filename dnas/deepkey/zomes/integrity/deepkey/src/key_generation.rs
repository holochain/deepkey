use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyGeneration {
    pub new_key: AgentPubKey,
    pub new_key_signing_of_author: ActionHash,
}
pub fn validate_create_key_generation(
    _action: EntryCreationAction,
    _key_generation: KeyGeneration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_key_generation(
    _action: Update,
    _key_generation: KeyGeneration,
    _original_action: EntryCreationAction,
    _original_key_generation: KeyGeneration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Key Generations cannot be updated"),
        ),
    )
}
pub fn validate_delete_key_generation(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_key_generation: KeyGeneration,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Key Generations cannot be deleted"),
        ),
    )
}
