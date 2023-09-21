use deepkey_integrity::*;
use hdk::prelude::*;

// This returns Private Entries and Metadata for the registered keys.
// It is the more sensitive version of the public `query_keyset_keys` in keyset_root.rs
#[hdk_extern]
pub fn query_local_key_info(_: ()) -> ExternResult<Vec<KeyRegistration>> {
    let app_entry_def: AppEntryDef = UnitEntryTypes::DnaBinding.try_into()?;
    let entry_type: EntryType = EntryType::App(app_entry_def);

    let records = query(
        ChainQueryFilter::new()
            .entry_type(entry_type)
            .include_entries(true),
    )?;

    Err(wasm_error!(WasmErrorInner::Guest("ahhh".into())))
}
