use hdi::prelude::{*, holo_hash::hash_type::Dna};

#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DnaBinding {
    pub key_meta: ActionHash, // Referencing a KeyMeta by its ActionHash
    pub dna_hash: HoloHash<Dna>, //The hash of the DNA the key is bound to
    pub app_name: String,
}

pub fn validate_create_dna_binding(
    _action: EntryCreationAction,
    _dna_binding: DnaBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_dna_binding(
    _action: Update,
    _dna_binding: DnaBinding,
    _original_action: EntryCreationAction,
    _original_dna_binding: DnaBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_dna_binding(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_dna_binding: DnaBinding,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
