use deepkey_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn instantiate_key_generation(new_key: AgentPubKey) -> ExternResult<KeyGeneration> {
    let chain_key = agent_info()?.agent_latest_pubkey;
    let signature = sign(chain_key, new_key.clone())?;
    Ok(KeyGeneration {
        new_key,
        new_key_signing_of_author: signature,
    })
}
#[hdk_extern]
pub fn get_key_generation(key_generation_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(key_generation_hash, GetOptions::default())
}
