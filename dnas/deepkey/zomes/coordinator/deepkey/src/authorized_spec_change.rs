use hdk::prelude::*;
use deepkey_integrity::*;
#[hdk_extern]
pub fn create_authorized_spec_change(
    authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<Record> {
    let authorized_spec_change_hash = create_entry(
        &EntryTypes::AuthorizedSpecChange(authorized_spec_change.clone()),
    )?;
    let record = get(authorized_spec_change_hash.clone(), GetOptions::default())?
        .ok_or(
            wasm_error!(
                WasmErrorInner::Guest(String::from("Could not find the newly created AuthorizedSpecChange"))
            ),
        )?;
    Ok(record)
}
#[hdk_extern]
pub fn get_authorized_spec_change(
    authorized_spec_change_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get(authorized_spec_change_hash, GetOptions::default())
}
