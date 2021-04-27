use hdk::prelude::*;

#[cfg(test)]
use ::fixt::prelude::*;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const KEYSET_ROOT_INDEX: EntryDefIndex = EntryDefIndex(3);

/// KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.
pub const KEYSET_ROOT_CHAIN_INDEX: u32 = 3;

#[hdk_entry(id = "keyset_root")]
/// We need an entry to create a permanent anchor that can be used to reference the space of keys under the control of a human agent.
/// This is commited only by the FirstDeepkeyAgent (FDA) not later devices that are joining this same agency context.
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
        fda_pubkey_signed_by_root_key: Signature
    ) -> Self {
        Self {
            first_deepkey_agent,
            root_pub_key,
            fda_pubkey_signed_by_root_key,
        }
    }

    pub fn as_first_deepkey_agent_ref(&self) -> &AgentPubKey {
        &self.first_deepkey_agent
    }

    pub fn as_root_pub_key_ref(&self) -> &AgentPubKey {
        &self.root_pub_key
    }

    pub fn as_fda_pubkey_signed_by_root_key_ref(&self) -> &Signature {
        &self.fda_pubkey_signed_by_root_key
    }
}

#[cfg(test)]
fixturator!(
    KeysetRoot;
    constructor fn new(AgentPubKey, AgentPubKey, Signature);
);

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::KEYSET_ROOT_INDEX;
    use super::KeysetRoot;

    #[test]
    fn keyset_root_index_test() {
        assert_eq!(
            KEYSET_ROOT_INDEX,
            entry_def_index!(KeysetRoot).unwrap()
        )
    }
}