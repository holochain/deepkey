use hdk::prelude::*;
use crate::change_rule::error::Error;
use crate::keyset_root::entry::KeysetRoot;

pub const CHANGE_RULE_INDEX: EntryDefIndex = EntryDefIndex(0);

/// Represents an M:N multisignature spec.
/// The trivial case 1:1 represents a single agent to sign.
/// We need an entry to define the rules of authority (for authorizing or revoking) keys in the space under a KeysetRoot.
/// This is only committed by the FDA.
#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthoritySpec {
    /// // set to 1 for a single signer scenario
    sigs_required: u8,
    authorized_signers: Vec<AgentPubKey>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthorizedSpecChange {
    new_spec: AuthoritySpec,
    /// Signature of the content of the authority_spec field,
    /// signed by throwaway RootKey on Create,
    /// or according to previous AuthSpec upon Update.
    authorization_of_new_spec: Vec<Signature>,
}

#[hdk_entry(id = "change_rule")]
pub struct ChangeRule {
    keyset_root: HeaderHash,
    spec_change: AuthorizedSpecChange,
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::CHANGE_RULE_INDEX;
    use super::ChangeRule;

    #[test]
    fn change_rule_index_test() {
        assert_eq!(
            CHANGE_RULE_INDEX,
            entry_def_index!(ChangeRule)
        )
    }
}