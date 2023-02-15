use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct JoiningProof {
    pub keyset_proof: String,
    pub membrane_proof: String,
}
pub fn validate_create_joining_proof(
    _action: EntryCreationAction,
    _joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_joining_proof(
    _action: Update,
    _joining_proof: JoiningProof,
    _original_action: EntryCreationAction,
    _original_joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Joining Proofs cannot be updated")))
}
pub fn validate_delete_joining_proof(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from("Joining Proofs cannot be deleted")))
}
