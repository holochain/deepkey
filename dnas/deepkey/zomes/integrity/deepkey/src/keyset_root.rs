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
    // if *action.action_seq() != KEYSET_ROOT_INDEX {
    //     return Ok(ValidateCallbackResult::Invalid(
    //         "KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.".to_string(),
    //     ));
    // }

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
pub fn validate_delete_link_keyset_root_to_key_anchors(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "KeysetRootToKeyAnchors links cannot be deleted",
    )))
}
