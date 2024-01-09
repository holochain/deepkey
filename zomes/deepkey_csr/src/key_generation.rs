use hdk::prelude::*;

#[hdk_extern]
pub fn get_key_generation(key_generation_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(key_generation_hash, GetOptions::default())
}
