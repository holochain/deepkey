use hdi::prelude::*;

pub const KEYSET_ROOT_INDEX: u32 = POST_GENESIS_SEQ_THRESHOLD + 1;


#[hdk_entry_helper]
#[derive(Clone)]
pub struct KeysetRoot {
    pub first_deepkey_agent: AgentPubKey,
    /// The private key is thrown away.
    root_pub_key: [u8; 32],
    fda_pubkey_signed_by_root_key: Signature,
}

impl KeysetRoot {
    pub fn new(
        first_deepkey_agent: AgentPubKey,
        root_pub_key: [u8; 32],
        fda_pubkey_signed_by_root_key: Signature,
    ) -> Self {
        Self {
            first_deepkey_agent,
            root_pub_key,
            fda_pubkey_signed_by_root_key,
        }
    }
}
