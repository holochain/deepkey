use hdk::prelude::*;
use deepkey_integrity::*;
#[hdk_extern]
pub fn create_key_revocation(key_revocation: KeyRevocation) -> ExternResult<Record> {
    let key_revocation_hash = create_entry(
        &EntryTypes::KeyRevocation(key_revocation.clone()),
    )?;
    let record = get(key_revocation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created KeyRevocation"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_key_revocation(
    original_key_revocation_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get_latest_key_revocation(original_key_revocation_hash)
}
fn get_latest_key_revocation(
    key_revocation_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    let details = get_details(key_revocation_hash, GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest("KeyRevocation not found".into())))?;
    let record_details = match details {
        Details::Entry(_) => {
            Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
        }
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => get_latest_key_revocation(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateKeyRevocationInput {
    pub previous_key_revocation_hash: ActionHash,
    pub updated_key_revocation: KeyRevocation,
}
#[hdk_extern]
pub fn update_key_revocation(input: UpdateKeyRevocationInput) -> ExternResult<Record> {
    let updated_key_revocation_hash = update_entry(
        input.previous_key_revocation_hash,
        &input.updated_key_revocation,
    )?;
    let record = get(updated_key_revocation_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly updated KeyRevocation"))
            ),
        )?;
    Ok(record)
}
