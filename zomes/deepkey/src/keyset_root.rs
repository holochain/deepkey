use hdk::prelude::*;

#[hdk_entry(id = "keyset_root")]
pub struct KeysetRoot {
    first_deepkey_agent: AgentPubKey,
    root_pub_key: AgentPubKey,
    fda_pubkey_signed_by_root_key: Signature,
}

#[hdk_extern]
fn create_keyset_root(new_keyset_root: KeysetRoot) -> ExternResult<HeaderHash> {
    create_entry(new_keyset_root)
}