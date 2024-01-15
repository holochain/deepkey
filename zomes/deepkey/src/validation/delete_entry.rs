use crate::{
    EntryTypesUnit,
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_create_action,
    detect_app_entry_unit,
    // Macros
    valid, invalid,
};


pub fn validation(
    original_action_hash: ActionHash,
    _original_entry_hash: EntryHash,
    _delete: Delete
) -> ExternResult<ValidateCallbackResult> {
    let create = summon_create_action( &original_action_hash )?;

    match detect_app_entry_unit( &create )? {
        EntryTypesUnit::KeysetRoot => {
            // let create = summon_create_action( &original_action_hash )?;

            // if delete.author != create.author {
            //     invalid!(format!(
            //         "Not authorized to delete entry created by author {}",
            //         create.author
            //     ))
            // }

            invalid!(format!("Keyset Roots cannot be deleted"))
        },
        EntryTypesUnit::ChangeRule => {
            invalid!(format!("Change Rules cannot be deleted"))
        },
        EntryTypesUnit::DeviceInvite => {
            invalid!(format!("Device Invites cannot be deleted"))
        },
        EntryTypesUnit::DeviceInviteAcceptance => {
            invalid!(format!("Device Invite Acceptances cannot be deleted"))
        },
        EntryTypesUnit::KeyRegistration => {
            valid!()
        },
        EntryTypesUnit::KeyAnchor => {
            valid!()
        },
        EntryTypesUnit::KeyMeta => {
            valid!()
        },
        EntryTypesUnit::DnaBinding => {
            valid!()
        },
        // entry_type_unit => invalid!(format!("Delete validation not implemented for entry type: {:?}", entry_type_unit )),
    }
}
