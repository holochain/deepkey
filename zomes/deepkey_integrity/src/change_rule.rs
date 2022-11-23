use hdi::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const CHANGE_RULE_INDEX: EntryDefIndex = EntryDefIndex(0);

/// Represents an M:N multisignature spec.
/// The trivial case 1:1 represents a single agent to sign.
/// We need an entry to define the rules of authority
/// (for authorizing or revoking) keys in the space under a KeysetRoot.
/// This is only committed by the FDA.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct AuthoritySpec {
    /// set to 1 for a single signer scenario
    pub sigs_required: u8,
    /// These signers probably do NOT exist on the DHT.
    /// E.g. a revocation key used to create the first change rule.
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

pub type Authorization = (u8, Signature);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AuthorizedSpecChange {
    pub new_spec: AuthoritySpec,
    /// Signature of the content of the authority_spec field,
    /// signed by throwaway RootKey on Create,
    /// or according to previous AuthSpec upon Update.
    pub authorization_of_new_spec: Vec<Authorization>,
}

impl AuthorizedSpecChange {
    pub fn new(new_spec: AuthoritySpec, authorization_of_new_spec: Vec<Authorization>) -> Self {
        Self {
            new_spec,
            authorization_of_new_spec,
        }
    }
    pub fn as_new_spec_ref(&self) -> &AuthoritySpec {
        &self.new_spec
    }
    pub fn as_authorization_of_new_spec_ref(&self) -> &Vec<Authorization> {
        &self.authorization_of_new_spec
    }
}

// #[hdk_entry(id = "change_rule", required_validation_type = "full")]
// The author needs to be linked from the KeysetRoot
// #[derive(Clone)]
#[hdk_entry_helper]
pub struct ChangeRule {
    pub keyset_root: ActionHash,
    pub keyset_leaf: ActionHash,
    pub spec_change: AuthorizedSpecChange,
}

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

    pub fn as_keyset_leaf_ref(&self) -> &ActionHash {
        &self.keyset_leaf
    }

    pub fn as_keyset_root_ref(&self) -> &ActionHash {
        &self.keyset_root
    }

    pub fn as_spec_change_ref(&self) -> &AuthorizedSpecChange {
        &self.spec_change
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
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Element missing its ChangeRule")]
    EntryMissing,

    #[error("Attempted to delete a ChangeRule")]
    DeleteAttempted,

    #[error("Attempted to update a ChangeRule")]
    UpdateAttempted,

    #[error("The ChangeRule author is not the FDA on the KeysetRoot")]
    AuthorNotFda,

    #[error("Multiple creation signatures found")]
    MultipleCreateSignatures,

    #[error("No creation signature found")]
    NoCreateSignature,

    #[error("Invalid creation signature")]
    BadCreateSignature,

    #[error("The new ChangeRule has a different KeysetRoot")]
    KeysetRootMismatch,

    #[error("The new ChangeRule has the wrong number of signatures")]
    WrongNumberOfSignatures,

    #[error("The new ChangeRule referenced an authorizor position that doesn't exist")]
    AuthorizedPositionOutOfBounds,

    #[error("The new ChangeRule references a KeysetLeaf that is incompatible with its KeysetRoot")]
    BadKeysetLeaf,

    #[error("The new ChangeRule references a stale keyset leaf")]
    StaleKeysetLeaf,

    #[error("The new ChangeRule has no validation package")]
    MissingValidationPackage,

    #[error("The new ChangeRule has an invalid signature")]
    BadUpdateSignature,

    #[error(
        "The new ChangeRule has fewer authorized signers than the minimum required signatures"
    )]
    NotEnoughSigners,

    #[error("The new ChangeRule requires zero signatures")]
    NotEnoughSignatures,

    #[error("The new ChangeRule update does not reference the root ChangeRule")]
    BranchingUpdates,

    #[error("The ChangeRule created does not immediately follow its KeysetRoot")]
    CreateNotAfterKeysetRoot,

    #[error("The ChangeRule element has the wrong header")]
    WrongHeader,

    #[error("Wasm error {0}")]
    Wasm(WasmError),
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
    }
}

impl From<Error> for ExternResult<ValidateCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}

impl From<WasmError> for Error {
    fn from(e: WasmError) -> Error {
        Error::Wasm(e)
    }
}
