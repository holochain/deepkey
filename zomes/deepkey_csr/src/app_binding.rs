use crate::utils;
use deepkey::*;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
};


// This returns AppBindings on the local chain.
#[hdk_extern]
pub fn query_app_bindings(_: ()) -> ExternResult<Vec<AppBinding>> {
    Ok(
        utils::query_entry_type( EntryTypesUnit::AppBinding )?
            .into_iter()
            .filter_map( |record| AppBinding::try_from( record ).ok() )
            .collect()
    )
}


// This returns Private Entries and Metadata for the registered keys.
// It is the more sensitive version of the public `query_keyset_keys` in keyset_root.rs
#[hdk_extern]
pub fn query_key_info(_: ()) -> ExternResult<Vec<(AppBinding, KeyMeta, KeyRegistration)>> {
    Ok(
        query_app_bindings(())?
            .into_iter()
            .filter_map( |app_binding| {
                let key_meta_record = must_get( &app_binding.key_meta_addr ).ok()?;
                let key_meta = KeyMeta::try_from( key_meta_record ).ok()?;

                Some((app_binding, key_meta))
            })
            .filter_map( |(app_binding, key_meta)| {
                let key_anchor_record = must_get( &key_meta.key_anchor_addr ).ok()?;
                let key_reg_record = must_get( key_anchor_record.action().prev_action()? ).ok()?;
                // let key_reg_record = must_get( &key_meta.new_key ).ok()?;
                let key_reg = KeyRegistration::try_from( key_reg_record ).ok()?;

                Some((app_binding, key_meta, key_reg))
            })
            .collect()
    )
}
