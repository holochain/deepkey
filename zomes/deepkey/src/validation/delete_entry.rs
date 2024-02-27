use crate::{
    EntryTypesUnit,

    KeyAnchor,
    KeyRegistration,
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_app_entry,
    summon_creation_action,
    detect_app_entry_unit,

    // Macros
    valid, invalid,
};


pub fn validation(
    original_action_hash: ActionHash,
    _original_entry_hash: EntryHash,
    delete: Delete
) -> ExternResult<ValidateCallbackResult> {
    let creation = summon_creation_action( &original_action_hash )?;

    match detect_app_entry_unit( &creation )? {
        EntryTypesUnit::KeysetRoot => {
            invalid!(format!("Keyset Roots cannot be deleted"))
        },
        EntryTypesUnit::ChangeRule => {
            invalid!(format!("Change Rules cannot be deleted"))
        },
        EntryTypesUnit::KeyRegistration => {
            invalid!(format!("Key Registrations cannot be deleted"))
        },
        EntryTypesUnit::KeyAnchor => {
            // Check previous action is key registration revoke.
            let key_anchor_entry : KeyAnchor = summon_app_entry( &original_action_hash.into() )?;
            let key_reg : KeyRegistration = summon_app_entry( &delete.prev_action.into() )?;

            let key_rev = match key_reg {
                KeyRegistration::Delete(key_rev) => key_rev,
                _ => invalid!(format!(
                    "KeyAnchor update must be preceeded by a KeyRegistration::Delete"
                )),
            };

            let prior_key_reg : KeyRegistration = summon_app_entry(
                &key_rev.prior_key_registration.into()
            )?;

            if prior_key_reg.key_anchor()? != key_anchor_entry {
                invalid!(format!(
                    "Deleted KeyAnchor does not match prior KeyRegistration key anchor: {:#?} != {:#?}",
                    key_anchor_entry, prior_key_reg.key_anchor()?,
                ))
            }

            valid!()
        },
        EntryTypesUnit::KeyMeta => {
            invalid!(format!("Key Metas cannot be deleted"))
        },
        EntryTypesUnit::AppBinding => {
            invalid!(format!("App Bindings cannot be deleted"))
        },
        // entry_type_unit => invalid!(format!("Delete validation not implemented for entry type: {:?}", entry_type_unit )),
    }
}
