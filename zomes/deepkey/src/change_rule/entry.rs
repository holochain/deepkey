use hdk::prelude::*;
use crate::change_rule::error::Error;

#[cfg(test)]
use ::fixt::prelude::*;

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
    pub authorized_signers: Vec<AgentPubKey>,
}

#[cfg(test)]
fixturator!(
    AuthoritySpec;
    constructor fn new(U8, AgentPubKeyVec);
);

impl AuthoritySpec {
    pub fn new(sigs_required: u8, authorized_signers: Vec<AgentPubKey>) -> Self {
        Self {
            sigs_required,
            authorized_signers,
        }
    }
}

pub type Authorization = (u8, Signature);

#[cfg(test)]
pub fn new_authorization(position: u8, signature: Signature) -> Authorization {
    (position, signature)
}

#[cfg(test)]
fixturator!(
    with_vec 0 5;
    Authorization;
    vanilla fn new_authorization(U8, Signature);
);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AuthorizedSpecChange {
    pub new_spec: AuthoritySpec,
    /// Signature of the content of the authority_spec field,
    /// signed by throwaway RootKey on Create,
    /// or according to previous AuthSpec upon Update.
    pub authorization_of_new_spec: Vec<Authorization>,
}

#[cfg(test)]
fixturator!(
    AuthorizedSpecChange;
    constructor fn new(AuthoritySpec, AuthorizationVec);
);

impl AuthorizedSpecChange {
    pub fn new(new_spec: AuthoritySpec, authorization_of_new_spec: Vec<Authorization>) -> Self {
        Self { new_spec, authorization_of_new_spec }
    }
    pub fn as_new_spec_ref(&self) -> &AuthoritySpec {
        &self.new_spec
    }
    pub fn as_authorization_of_new_spec_ref(&self) -> &Vec<Authorization> {
        &self.authorization_of_new_spec
    }
}

#[hdk_entry(id = "change_rule")]
// The author needs to be linked from the KeysetRoot
#[derive(Clone)]
pub struct ChangeRule {
    pub keyset_root: HeaderHash,
    pub spec_change: AuthorizedSpecChange,
}

impl TryFrom<&Element> for ChangeRule {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        Ok(match element.entry() {
            ElementEntry::Present(serialized_change_rule) => match ChangeRule::try_from(serialized_change_rule) {
                Ok(change_rule) => change_rule,
                Err(e) => return Err(Error::Wasm(e)),
            }
            __ => return Err(Error::EntryMissing),
        })
    }
}

#[cfg(test)]
fixturator!(
    ChangeRule;
    constructor fn new(HeaderHash, AuthorizedSpecChange);
);

impl ChangeRule {
    pub fn new(keyset_root: HeaderHash, spec_change: AuthorizedSpecChange) -> Self {
        Self { keyset_root, spec_change }
    }

    pub fn as_keyset_root_ref(&self) -> &HeaderHash {
        &self.keyset_root
    }

    pub fn as_spec_change_ref(&self) -> &AuthorizedSpecChange {
        &self.spec_change
    }

    pub fn authorize(&self, authorization: &[Authorization], data: &[u8]) -> Result<(), Error> {
        if authorization.len() != self.spec_change.new_spec.sigs_required as usize {
            Err(Error::WrongNumberOfSignatures)
        }
        else {
            for (position, signature) in authorization.iter() {
                match self.spec_change.new_spec.authorized_signers.get(*position as usize) {
                    Some(agent) => {
                        trace!(?agent, ?signature, ?data);
                        if !verify_signature_raw(
                        agent.to_owned(),
                        signature.to_owned(),
                        data.to_vec()
                    )? {
                        // Short circuit any failed sig.
                        return Err(Error::BadUpdateSignature);
                    }},
                    None => return Err(Error::AuthorizedPositionOutOfBounds),
                }
            }
            Ok(())
        }
    }
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::CHANGE_RULE_INDEX;
    use super::ChangeRule;
    use ::fixt::prelude::*;
    use crate::change_rule::entry::AuthorizationFixturator;
    use crate::change_rule::entry::ChangeRuleFixturator;
    use crate::change_rule::error::Error;

    #[test]
    fn change_rule_index_test() {
        assert_eq!(
            CHANGE_RULE_INDEX,
            entry_def_index!(ChangeRule).unwrap()
        )
    }

    #[test]
    fn change_rule_authorize_test() {
        let mut change_rule = fixt!(ChangeRule);
        change_rule.spec_change.new_spec.sigs_required = 2;

        assert_eq!(
            Err(Error::WrongNumberOfSignatures),
            change_rule.authorize(&vec![fixt!(Authorization)], &[1, 2, 3]),
        );

        assert_eq!(
            Err(Error::AuthorizedPositionOutOfBounds),
            change_rule.authorize(&vec![(50, fixt!(Signature)), fixt!(Authorization)], &[1, 2, 3]),
        );

        change_rule.spec_change.new_spec.authorized_signers = vec![fixt!(AgentPubKey), fixt!(AgentPubKey), fixt!(AgentPubKey)];

        let authorization = vec![
            (1, fixt!(Signature)),
            (2, fixt!(Signature)),
        ];

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    change_rule.spec_change.new_spec.authorized_signers[1].clone(),
                    authorization[0].1.clone(),
                    vec![1, 2, 3],
                )
            ))
            .times(1)
            .return_const(Ok(false));

        set_hdk(mock_hdk);

        assert_eq!(
            Err(Error::BadUpdateSignature),
            change_rule.authorize(&authorization, &[1, 2, 3]),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    change_rule.spec_change.new_spec.authorized_signers[1].clone(),
                    authorization[0].1.clone(),
                    vec![1, 2, 3],
                )
            ))
            .times(1)
            .return_const(Ok(true));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    change_rule.spec_change.new_spec.authorized_signers[2].clone(),
                    authorization[1].1.clone(),
                    vec![1, 2, 3],
                )
            ))
            .times(1)
            .return_const(Ok(true));

        set_hdk(mock_hdk);

        assert_eq!(
            Ok(()),
            change_rule.authorize(&authorization, &[1, 2, 3]),
        );
    }
}