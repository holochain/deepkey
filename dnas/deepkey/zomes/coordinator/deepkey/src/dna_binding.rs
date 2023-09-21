use deepkey_integrity::*;
use hdk::prelude::*;

// This returns Private Entries and Metadata for the registered keys.
// It is the more sensitive version of the public `query_keyset_keys` in keyset_root.rs
#[hdk_extern]
pub fn query_local_key_info(_: ()) -> ExternResult<Vec<(DnaBinding, KeyMeta, KeyRegistration)>> {
    let app_entry_def: AppEntryDef = UnitEntryTypes::DnaBinding.try_into()?;
    let entry_type: EntryType = EntryType::App(app_entry_def);

    let local_key_infos = query(
        ChainQueryFilter::new()
            .entry_type(entry_type)
            .include_entries(true),
    )?
    .into_iter()
    .map(|dna_binding_record| {
        dna_binding_record
            .entry
            .to_app_option::<DnaBinding>()
            .map_err(|err| {
                wasm_error!(WasmErrorInner::Guest(format!(
                    "Error deserializing DnaBinding. {:?}",
                    err
                )))
            })
    })
    .collect::<ExternResult<Vec<Option<DnaBinding>>>>()?
    .into_iter()
    .filter_map(|x| x)
    .map(|dna_binding| {
        let key_meta = get(dna_binding.key_meta.clone(), GetOptions::default())?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Cannot find associated key meta from the dna binding record."
            ))))?
            .entry
            .to_app_option::<KeyMeta>()
            .map_err(|err| {
                wasm_error!(WasmErrorInner::Guest(format!(
                    "Error deserializing KeyMeta. {:?}",
                    err
                )))
            })?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Cannot find deserialized key meta from the dna binding record."
            ))))?;

        Ok((dna_binding, key_meta))
    })
    .collect::<ExternResult<Vec<(DnaBinding, KeyMeta)>>>()?
    .into_iter()
    .map(|(dna_binding, key_meta)| {
        let key_registration = get(key_meta.new_key.clone(), GetOptions::default())?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Cannot find associated key registration from the key meta record."
            ))))?
            .entry
            .to_app_option::<KeyRegistration>()
            .map_err(|err| {
                wasm_error!(WasmErrorInner::Guest(format!(
                    "Error deserializing KeyRegistration. {:?}",
                    err
                )))
            })?
            .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
                "Cannot find deserialized key registration from the key meta record."
            ))))?;

        Ok((dna_binding, key_meta, key_registration))
    })
    .collect::<ExternResult<Vec<(DnaBinding, KeyMeta, KeyRegistration)>>>()?;
    Ok(local_key_infos)
}
