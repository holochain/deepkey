use crate::utils;
use deepkey::*;
use hdk::prelude::*;
use hdk_extensions::{
    hdi_extensions::{
        guest_error,
    },
};


#[hdk_extern]
pub fn query_key_metas_for_app_binding(addr: ActionHash) -> ExternResult<Vec<KeyMeta>> {
    Ok(
        utils::query_entry_type( EntryTypesUnit::KeyMeta )?
            .into_iter()
            .filter_map( |record| KeyMeta::try_from( record ).ok() )
            .filter( |key_meta| key_meta.app_binding_addr == addr )
            .collect()
    )
}


#[hdk_extern]
pub fn query_key_meta_records(_: ()) -> ExternResult<Vec<Record>> {
    utils::query_entry_type( EntryTypesUnit::KeyMeta )
}


#[hdk_extern]
pub fn query_key_metas(_: ()) -> ExternResult<Vec<KeyMeta>> {
    Ok(
        utils::query_entry_type( EntryTypesUnit::KeyMeta )?
            .into_iter()
            .filter_map( |record| KeyMeta::try_from( record ).ok() )
            .collect()
    )
}


#[hdk_extern]
pub fn query_key_meta_for_key(anchor_addr: ActionHash) -> ExternResult<KeyMeta> {
    query_key_metas(())?
        .into_iter()
        .find( |key_meta| key_meta.key_anchor_addr == anchor_addr  )
        .ok_or(guest_error!(format!("No KeyMeta for anchor addr: {}", anchor_addr )))
}


#[hdk_extern]
pub fn query_key_meta_for_registration(key_reg_addr: ActionHash) -> ExternResult<KeyMeta> {
    query_key_metas(())?
        .into_iter()
        .find( |key_meta| key_meta.key_registration_addr == key_reg_addr  )
        .ok_or(guest_error!(format!("No KeyMeta for registration addr: {}", key_reg_addr )))
}
