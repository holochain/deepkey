use crate::{
    EntryTypes,

    KeysetRoot,
    DeviceInvite,
};

use hdi::prelude::*;
use hdi_extensions::{
    verify_app_entry_struct,
    // Macros
    valid, // invalid,
};


pub fn validation(
    app_entry: EntryTypes,
    _create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(_ksr_entry) => {
            valid!()
        },
        EntryTypes::ChangeRule(_change_rule_entry) => {
            valid!()
        },
        EntryTypes::DeviceInvite(device_invite_entry) => {
            verify_app_entry_struct::<KeysetRoot>( &device_invite_entry.keyset_root.into() )?;

            valid!()
        },
        EntryTypes::DeviceInviteAcceptance(device_invite_acceptance_entry) => {
            verify_app_entry_struct::<DeviceInvite>( &device_invite_acceptance_entry.invite.into() )?;

            valid!()
        },
        EntryTypes::KeyRegistration(_key_registration_entry) => {
            valid!()
        },
        EntryTypes::KeyAnchor(_key_anchor_entry) => {
            valid!()
        },
        EntryTypes::KeyMeta(_key_meta_entry) => {
            valid!()
        },
        EntryTypes::DnaBinding(_dna_binding_entry) => {
            valid!()
        },
        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}
