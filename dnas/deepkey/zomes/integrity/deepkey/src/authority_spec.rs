use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AuthoritySpec {
    pub sigs_required: u32,
    pub signers: Vec<AgentPubKey>,
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
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Authority Specs cannot be updated"),
        ),
    )
}
pub fn validate_delete_authority_spec(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_authority_spec: AuthoritySpec,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Authority Specs cannot be deleted"),
        ),
    )
}
pub fn validate_create_link_signer_to_authority_specs(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _authority_spec: crate::AuthoritySpec = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Linked action must reference an entry"))
            ),
        )?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_signer_to_authority_specs(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("SignerToAuthoritySpecs links cannot be deleted"),
        ),
    )
}
