use hdi::prelude::*;

use crate::{Authorization, AuthorizedSpecChange, error::Error};

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
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _change_rule: crate::ChangeRule = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _change_rule: crate::ChangeRule = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
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

/*



use hdi::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
// pub const CHANGE_RULE_INDEX: EntryDefIndex = EntryDefIndex(0);

// impl TryFrom<&Record> for ChangeRule {
//     type Error = Error;
//     fn try_from(element: &Record) -> Result<Self, Self::Error> {
//         Ok(match element.entry() {
//             RecordEntry::Present(serialized_change_rule) => {
//                 match ChangeRule::try_from(serialized_change_rule) {
//                     Ok(change_rule) => change_rule,
//                     Err(e) => return Err(Error::Wasm(e)),
//                 }
//             }
//             __ => return Err(Error::EntryMissing),
//         })
//     }
// }

// #[cfg(test)]
// use ::fixt::prelude::*;

// #[cfg(test)]
// fixturator!(
//     AuthoritySpec;
//     constructor fn new(U8, AgentPubKeyVec);
// );

// #[cfg(test)]
// pub fn new_authorization(position: u8, signature: Signature) -> Authorization {
//     (position, signature)
// }

// #[cfg(test)]
// fixturator!(
//     with_vec 0 5;
//     Authorization;
//     vanilla fn new_authorization(U8, Signature);
// );

// #[cfg(test)]
// fixturator!(
//     AuthorizedSpecChange;
//     constructor fn new(AuthoritySpec, AuthorizationVec);
// );

// #[cfg(test)]
// fixturator!(
//     ChangeRule;
//     constructor fn new(ActionHash, ActionHash, AuthorizedSpecChange);
// );

// #[cfg(test)]
// pub mod tests {
//     use hdk::prelude::*;
//     use super::CHANGE_RULE_INDEX;
//     use super::ChangeRule;
//     use ::fixt::prelude::*;
//     use crate::change_rule::entry::AuthorizationFixturator;
//     use crate::change_rule::entry::ChangeRuleFixturator;
//     use crate::change_rule::error::Error;

//     #[test]
//     fn change_rule_index_test() {
//         assert_eq!(
//             CHANGE_RULE_INDEX,
//             entry_def_index!(ChangeRule).unwrap()
//         )
//     }

//     #[test]
//     fn change_rule_authorize_test() {
//         let mut change_rule = fixt!(ChangeRule);
//         change_rule.spec_change.new_spec.sigs_required = 2;

//         assert_eq!(
//             Err(Error::WrongNumberOfSignatures),
//             change_rule.authorize(&vec![fixt!(Authorization)], &[1, 2, 3]),
//         );

//         assert_eq!(
//             Err(Error::AuthorizedPositionOutOfBounds),
//             change_rule.authorize(&vec![(50, fixt!(Signature)), fixt!(Authorization)], &[1, 2, 3]),
//         );

//         change_rule.spec_change.new_spec.authorized_signers = vec![fixt!(AgentPubKey), fixt!(AgentPubKey), fixt!(AgentPubKey)];

//         let authorization = vec![
//             (1, fixt!(Signature)),
//             (2, fixt!(Signature)),
//         ];

//         let mut mock_hdk = MockHdkT::new();

//         mock_hdk.expect_verify_signature()
//             .with(mockall::predicate::eq(
//                 VerifySignature::new_raw(
//                     change_rule.spec_change.new_spec.authorized_signers[1].clone(),
//                     authorization[0].1.clone(),
//                     vec![1, 2, 3],
//                 )
//             ))
//             .times(1)
//             .return_const(Ok(false));

//         set_hdk(mock_hdk);

//         assert_eq!(
//             Err(Error::BadUpdateSignature),
//             change_rule.authorize(&authorization, &[1, 2, 3]),
//         );

//         let mut mock_hdk = MockHdkT::new();

//         mock_hdk.expect_verify_signature()
//             .with(mockall::predicate::eq(
//                 VerifySignature::new_raw(
//                     change_rule.spec_change.new_spec.authorized_signers[1].clone(),
//                     authorization[0].1.clone(),
//                     vec![1, 2, 3],
//                 )
//             ))
//             .times(1)
//             .return_const(Ok(true));

//         mock_hdk.expect_verify_signature()
//             .with(mockall::predicate::eq(
//                 VerifySignature::new_raw(
//                     change_rule.spec_change.new_spec.authorized_signers[2].clone(),
//                     authorization[1].1.clone(),
//                     vec![1, 2, 3],
//                 )
//             ))
//             .times(1)
//             .return_const(Ok(true));

//         set_hdk(mock_hdk);

//         assert_eq!(
//             Ok(()),
//             change_rule.authorize(&authorization, &[1, 2, 3]),
//         );
//     }
// }

// use hdk::prelude::*;

*/
