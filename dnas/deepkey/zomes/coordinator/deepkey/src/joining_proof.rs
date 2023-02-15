use hdk::prelude::*;
use deepkey_integrity::*;
#[hdk_extern]
pub fn create_joining_proof(joining_proof: JoiningProof) -> ExternResult<Record> {
    let joining_proof_hash = create_entry(
        &EntryTypes::JoiningProof(joining_proof.clone()),
    )?;
    let record = get(joining_proof_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created JoiningProof"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_joining_proof(
    joining_proof_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(joining_proof_hash, GetOptions::default())
}
