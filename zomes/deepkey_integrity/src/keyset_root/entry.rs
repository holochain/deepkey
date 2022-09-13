use hdk::prelude::*;

#[cfg(test)]
use ::fixt::prelude::*;

/// KeysetRoot must be the 6th entry on `FirstDeepkeyAgent`'s chain (immediately after JoiningProof)
pub const KEYSET_ROOT_CHAIN_INDEX: u32 = 5;

/// Has test coverage in case entry_defs! ever changes.
pub const KEYSET_ROOT_INDEX: EntryDefIndex = EntryDefIndex(KEYSET_ROOT_CHAIN_INDEX as u8);

//#[hdk_entry(id = "keyset_root")]
#[hdk_entry_helper]
/// We need an entry to create a permanent anchor that can be used to reference the space of keys
/// under the control of a human agent.  This is commited only by the FirstDeepkeyAgent (FDA) not
/// later devices that are joining this same agency context.
/// 
/// What purpose is served by signing the publicly known first_deepkey_agent public key using the
/// ephemeral root_pub_key private key?  All this proves is that there *was* such a private key and
/// that root_pub_key was its public key.  The root_pub_key could conceivably be any random number,
/// and this proof gives it no more or less utility.  However, this Signature disallows any
/// pre-selected (ie. not random) root_pub_key (such as one owned by someone else) to be used --
/// since it is impossible to deduce such a public key's private signing key from that public key.
/// In fact, nothing prevents us from using the first_deepkey_agent public AgentPubKey and its
/// (non-ephemeral) private key as the root_pub_key.
#[derive(Clone)]
pub struct KeysetRoot {
    pub first_deepkey_agent: AgentPubKey,
    /// The private key is thrown away.
    pub root_pub_key: AgentPubKey,
    pub fda_pubkey_signed_by_root_key: Signature,
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
