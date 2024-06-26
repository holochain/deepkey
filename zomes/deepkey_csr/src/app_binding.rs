use crate::utils;
use deepkey::*;
use serde_bytes::ByteArray;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
    },
};


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
pub fn query_next_app_index(_: ()) -> ExternResult<u32> {
    Ok( query_app_bindings(())?.len() as u32 )
}

#[hdk_extern]
pub fn query_app_binding_by_index(index: u32) -> ExternResult<(ActionHash, AppBinding)> {
    query_app_bindings(())?
        .into_iter()
        .find( |(_, app_binding)| app_binding.app_index == index  )
        .ok_or(guest_error!(format!("No AppBinding with index: {}", index )))
}

#[hdk_extern]
pub fn query_app_binding_by_action(addr: ActionHash) -> ExternResult<AppBinding> {
    Ok(
        query_app_bindings(())?
            .into_iter()
            .find( |(app_binding_addr, _)| *app_binding_addr == addr  )
            .ok_or(guest_error!(format!("No AppBinding with action hash: {}", addr )))?.1
    )
}

#[hdk_extern]
pub fn query_app_bindings_by_installed_app_id(installed_app_id: String) -> ExternResult<Vec<(ActionHash, AppBinding)>> {
    Ok(
        query_app_bindings(())?
            .into_iter()
            .filter( |(_, app_binding)| app_binding.installed_app_id == installed_app_id  )
            .collect()
    )
}

#[hdk_extern]
pub fn query_app_binding_by_key(key_bytes: ByteArray<32>) -> ExternResult<(ActionHash, AppBinding)> {
    let key_meta = crate::key_meta::query_key_meta_for_key( key_bytes )?;
    let app_binding = query_app_binding_by_action( key_meta.app_binding_addr.clone() )?;

    debug!("Found AppBinding ({}) for KeyBytes: {:?}", key_meta.app_binding_addr, key_bytes );
    Ok((key_meta.app_binding_addr, app_binding))
}


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
