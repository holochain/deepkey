use crate::{
    EntryTypes,
    // EntryTypesUnit,

    KeyMeta,

    utils,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};


pub fn key_metas_for_app_index (
    author: &AgentPubKey,
    chain_top: &ActionHash,
    app_index: u32,
) -> ExternResult<Option<KeyMeta>> {
    let key_metas = get_activity_for_entry_type(
        EntryTypesUnit::KeyMeta,
        author,
        chain_top,
    )?
        .into_iter()
        .filter_map( |activity| {
            let key_meta : KeyMeta = match activity.cached_entry {
                Some(entry) => entry,
                None => must_get_entry(
                    activity.action.action().entry_hash().unwrap().to_owned()
                ).ok()?.content,
            }.try_into().ok()?;

            Some(( activity, key_meta ))
        })
        .filter( |(_, key_meta)|  )
        .collect();
}


pub fn validation(
    app_entry: EntryTypes,
    create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeyMeta(key_meta_entry) => {
            // Check that the app index exists

            // Check that the key index is incremented by 1
            let prev_key_meta = utils::prev_key_meta(
                &create.author,
                &create.prev_action,
                app_binding.app_index,
            )?;

            match prev_key_meta {
                Some(prev_key_meta) => {
                    if (prev_key_meta.key_index + 1) != key_meta_entry.key_index {
                        invalid!(format!(
                            "Key Meta for App Binding (index: {}) should have key_index: {}",
                            app_binding.app_index, prev_key_meta.key_index + 1,
                        ))
                    }
                },
                None => {
                    if key_meta_entry.key_index != 0 {
                        invalid!("First Key Meta for an App Binding should have key_index: 0".to_string())
                    }
                },
            }

            valid!()
        }
    }
}

