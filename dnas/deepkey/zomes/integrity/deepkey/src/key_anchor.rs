use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyAnchor {
    pub bytes: [u8; 32],
}
pub fn validate_create_key_anchor(
    _action: EntryCreationAction,
    _key_anchor: KeyAnchor,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_key_anchor(
    _action: Update,
    _key_anchor: KeyAnchor,
    _original_action: EntryCreationAction,
    _original_key_anchor: KeyAnchor,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_key_anchor(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_key_anchor: KeyAnchor,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

impl KeyAnchor {
    pub fn from_agent_key(agent_key: AgentPubKey) -> Self {
        let slice = agent_key.get_raw_32();
        let bytes: [u8; 32] = match slice.try_into() {
            Ok(array) => array,
            Err(_) => panic!("Failed to convert AgentPubKey to [u8; 32]"),
        };
        Self { bytes }
    }
}
