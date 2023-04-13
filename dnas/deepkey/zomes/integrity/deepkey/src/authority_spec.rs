use hdi::prelude::*;
// Represents an M:N multisignature spec.
// The trivial case 1:1 represents a single agent to sign.
// We need an entry to define the rules of authority
// (for authorizing or revoking) keys in the space under a KeysetRoot.
// This is only committed by the first Deepkey agent.
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AuthoritySpec {
    // set to 1 for a single signer scenario
    pub sigs_required: u8,
    // These signers may not exist on the DHT.
    // E.g. a revocation key used to create the first change rule.
    pub authorized_signers: Vec<AgentPubKey>,
}
impl AuthoritySpec {
    pub fn new(sigs_required: u8, authorized_signers: Vec<AgentPubKey>) -> Self {
        Self {
            sigs_required,
            authorized_signers,
        }
    }
}
pub fn validate_create_authority_spec(
    _action: EntryCreationAction,
    _authority_spec: AuthoritySpec,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_authority_spec(
    _action: Update,
    _authority_spec: AuthoritySpec,
    _original_action: EntryCreationAction,
    _original_authority_spec: AuthoritySpec,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Authority Specs cannot be updated",
    )))
}
pub fn validate_delete_authority_spec(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_authority_spec: AuthoritySpec,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Authority Specs cannot be deleted",
    )))
}
// pub fn validate_create_link_signer_to_authority_specs(
//     _action: CreateLink,
//     _base_address: AnyLinkableHash,
//     target_address: AnyLinkableHash,
//     _tag: LinkTag,
// ) -> ExternResult<ValidateCallbackResult> {
//     let action_hash = ActionHash::from(target_address);
//     let record = must_get_valid_record(action_hash)?;
//     let _authority_spec: crate::AuthoritySpec = record
//         .entry()
//         .to_app_option()
//         .map_err(|e| wasm_error!(e))?
//         .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
//             "Linked action must reference an entry"
//         ))))?;
//     Ok(ValidateCallbackResult::Valid)
// }
pub fn validate_delete_link_signer_to_authority_specs(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "SignerToAuthoritySpecs links cannot be deleted",
    )))
}
