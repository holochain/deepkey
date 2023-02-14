use hdi::prelude::*;

pub const KEYSET_ROOT_INDEX: u32 = POST_GENESIS_SEQ_THRESHOLD + 1;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct KeysetRoot {
    pub first_deepkey_agent: AgentPubKey,
    /// The private key is thrown away.
    root_pub_key: AgentPubKey,
    fda_pubkey_signed_by_root_key: Signature,
}
impl KeysetRoot {
    pub fn new(
        first_deepkey_agent: AgentPubKey,
        root_pub_key: AgentPubKey,
        fda_pubkey_signed_by_root_key: Signature,
    ) -> Self {
        Self {
            first_deepkey_agent,
            root_pub_key,
            fda_pubkey_signed_by_root_key,
        }
    }
}

pub fn validate_create_keyset_root(
    _action: EntryCreationAction,
    _keyset_root: KeysetRoot,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_keyset_root(
    _action: Update,
    _keyset_root: KeysetRoot,
    _original_action: EntryCreationAction,
    _original_keyset_root: KeysetRoot,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Keyset Roots cannot be updated",
    )))
}
pub fn validate_delete_keyset_root(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_keyset_root: KeysetRoot,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Keyset Roots cannot be deleted",
    )))
}

/*
////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
Entry struct definitions with necessary impls
pub const KEYSET_ROOT_INDEX: EntryDefIndex = EntryDefIndex(3);
pub const KEYSET_ROOT_INDEX: u32 = POST_GENESIS_SEQ_THRESHOLD + 1;

/// KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.
// pub const KEYSET_ROOT_CHAIN_INDEX: u32 = 3;

/// We need an entry to create a permanent anchor that can be used to reference the space of keys under the control of a human agent.
/// This is commited only by the FirstDeepkeyAgent (FDA) not later devices that are joining this same agency context.
pub fn validate_create_keyset_root(
    _keyset_root: KeysetRoot,
    action: EntryCreationAction,
) -> ExternResult<ValidateCallbackResult> {
    if *action.action_seq() != KEYSET_ROOT_INDEX {
        return Ok(ValidateCallbackResult::Invalid(
            "KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.".to_string(),
        ));
    }

    return Ok(ValidateCallbackResult::Valid);
}

*/
