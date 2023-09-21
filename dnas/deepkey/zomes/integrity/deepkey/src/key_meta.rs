use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub enum KeyType {
    AppUI,
    AppSig,
    AppEncryption,
    TLS,
}

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyMeta {
    pub new_key: ActionHash, // Referencing a KeyRegistration by its ActionHash
    pub derivation_path: [u8; 32],
    pub derivation_index: u32,
    pub key_type: KeyType,
}

pub fn validate_create_key_meta(
    _action: EntryCreationAction,
    _key_meta: KeyMeta,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_key_meta(
    _action: Update,
    _key_anchor: KeyMeta,
    _original_action: EntryCreationAction,
    _original_key_meta: KeyMeta,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_key_meta(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_key_meta: KeyMeta,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
