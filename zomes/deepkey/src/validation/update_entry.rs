use crate::{
    EntryTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};


pub fn validation(
    app_entry: EntryTypes,
    _update: Update,
    _original_action_hash: ActionHash,
    _original_entry_hash: EntryHash
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(_) => {
            invalid!(format!("Keyset Roots cannot be updated"))
        },
        EntryTypes::ChangeRule(_) => {
            valid!()
        },
        EntryTypes::DeviceInvite(_device_invite_entry) => {
            invalid!(format!("Device Invites cannot be updated"))
        },
        EntryTypes::DeviceInviteAcceptance(_device_invite_acceptance_entry) => {
            invalid!(format!("Device Invite Acceptances cannot be updated"))
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
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}
