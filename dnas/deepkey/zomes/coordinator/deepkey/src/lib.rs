pub mod change_rule;

use deepkey_integrity::{keyset_root::KeysetRoot, EntryTypes};
use hdk::prelude::*;

// The joining proof is added to the chain before init.
// const JOINING_PROOF_CHAIN_INDEX: u32 = 2;

#[hdk_entry_helper]
enum SerializeTypes {
    AgentPubKey(AgentPubKey),
}

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // Assume creating a new Keyset Space; thus we create a new KSR
    let fda: AgentPubKey = agent_info()?.agent_latest_pubkey;
    // make throwaway keypair
    let sigs = sign_ephemeral([fda.clone()].into()).unwrap();
    let root_pub_key = sigs.key;
    let sig = sigs.signatures.into_iter().next().ok_or_else(|| {
        wasm_error!(WasmErrorInner::Guest(
            "Expected an ephemeral signature".into()
        ))
    })?;

    let ksr = KeysetRoot::new(fda, root_pub_key, sig);
    create_entry(EntryTypes::KeysetRoot(ksr))?;
    Ok(InitCallbackResult::Pass)
}
