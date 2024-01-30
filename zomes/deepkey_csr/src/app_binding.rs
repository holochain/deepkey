use crate::utils;
use deepkey::*;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
    },
};


// This returns AppBindings on the local chain.
#[hdk_extern]
pub fn query_app_binding_records(_: ()) -> ExternResult<Vec<Record>> {
    utils::query_entry_type( EntryTypesUnit::AppBinding )
}

#[hdk_extern]
pub fn query_app_bindings(_: ()) -> ExternResult<Vec<(ActionHash, AppBinding)>> {
    Ok(
        query_app_binding_records(())?
            .into_iter()
            .filter_map( |record| Some((
                record.action_address().to_owned(),
                AppBinding::try_from( record ).ok()?
            )))
            .collect()
    )
}

#[hdk_extern]
pub fn query_app_binding_for_index(index: u32) -> ExternResult<(ActionHash, AppBinding)> {
    query_app_bindings(())?
        .into_iter()
        .find( |(_, app_binding)| app_binding.app_index == index  )
        .ok_or(guest_error!(format!("No AppBinding for index: {}", index )))
}


// This returns Private Entries and Metadata for the registered keys.
// It is the more sensitive version of the public `query_keyset_keys` in keyset_root.rs

type KeyInfo = (KeyMeta, KeyRegistration);
type AppKeyInfo = (AppBinding, Vec<KeyInfo>);

#[hdk_extern]
pub fn query_key_info(_: ()) -> ExternResult<Vec<AppKeyInfo>> {
    Ok(
        query_app_binding_records(())?
            .into_iter()
            .filter_map( |record| {
                let app_binding_addr = record.action_address().to_owned();
                let app_binding = AppBinding::try_from( record ).ok()?;

                let key_infos = crate::key_meta::query_key_metas_for_app_binding( app_binding_addr ).ok()?
                    .into_iter()
                    .filter_map( |key_meta| {
                        let key_reg_addr = key_meta.key_registration_addr.clone();

                        Some((
                            key_meta,
                            KeyRegistration::try_from(
                                must_get( &key_reg_addr ).ok()?
                            ).ok()?
                        ))
                    })
                    .collect::<Vec<KeyInfo>>();

                Some((app_binding, key_infos))
            })
            .collect::<Vec<AppKeyInfo>>()
    )
}
