use hdi::prelude::*;

use crate::{error::Error, Authorization, AuthorizedSpecChange};

// The author needs to be linked from the KeysetRoot
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ChangeRule {
    pub keyset_root: ActionHash,
    pub keyset_leaf: ActionHash,
    pub spec_change: AuthorizedSpecChange,
}

impl ChangeRule {
    pub fn new(
        keyset_root: ActionHash,
        keyset_leaf: ActionHash,
        spec_change: AuthorizedSpecChange,
    ) -> Self {
        Self {
            keyset_root,
            keyset_leaf,
            spec_change,
        }
    }

    pub fn authorize(&self, authorization: &[Authorization], data: &[u8]) -> Result<(), Error> {
        if authorization.len() != self.spec_change.new_spec.sigs_required as usize {
            Err(Error::WrongNumberOfSignatures)
        } else {
            for (position, signature) in authorization.iter() {
                match self
                    .spec_change
                    .new_spec
                    .authorized_signers
                    .get(*position as usize)
                {
                    Some(agent) => {
                        if !verify_signature_raw(
                            agent.to_owned(),
                            signature.to_owned(),
                            data.to_vec(),
                        )? {
                            // Short circuit any failed sig.
                            return Err(Error::BadUpdateSignature);
                        }
                    }
                    None => return Err(Error::AuthorizedPositionOutOfBounds),
                }
            }
            Ok(())
        }
    }
}

pub fn validate_create_change_rule(
    _action: EntryCreationAction,
    _change_rule: ChangeRule,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_change_rule(
    _action: Update,
    _change_rule: ChangeRule,
    _original_action: EntryCreationAction,
    _original_change_rule: ChangeRule,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_change_rule(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_change_rule: ChangeRule,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Change Rules cannot be deleted",
    )))
}
pub fn validate_create_link_change_rule_updates(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // let action_hash = ActionHash::from(base_address);
    // let record = must_get_valid_record(action_hash)?;
    // let _change_rule: crate::ChangeRule = record
    //     .entry()
    //     .to_app_option()
    //     .map_err(|e| wasm_error!(e))?
    //     .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
    //         "Linked action must reference an entry"
    //     ))))?;
    // let action_hash = ActionHash::from(target_address);
    // let record = must_get_valid_record(action_hash)?;
    // let _change_rule: crate::ChangeRule = record
    //     .entry()
    //     .to_app_option()
    //     .map_err(|e| wasm_error!(e))?
    //     .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
    //         "Linked action must reference an entry"
    //     ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_change_rule_updates(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "ChangeRuleUpdates links cannot be deleted",
    )))
}
