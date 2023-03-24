use hdk::prelude::*;
use deepkey_integrity::*;
#[hdk_extern]
pub fn create_key_generation(key_generation: KeyGeneration) -> ExternResult<Record> {
    let key_generation_hash = create_entry(
        &EntryTypes::KeyGeneration(key_generation.clone()),
    )?;
    let record = get(key_generation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created KeyGeneration"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_key_generation(
    key_generation_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(key_generation_hash, GetOptions::default())
}
