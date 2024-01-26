use crate::{
    EntryTypes,
    EntryTypesUnit,

    utils,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};


pub fn validation(
    app_entry: EntryTypes,
    update: Update,
    _original_action_hash: ActionHash,
    _original_entry_hash: EntryHash
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(_) => {
            invalid!(format!("Keyset Roots cannot be updated"))
        },
        EntryTypes::ChangeRule(change_rule_entry) => {
            let new_authority_spec = change_rule_entry.spec_change.new_spec;
            // Cannot require more signatures than there are authorities
            if new_authority_spec.sigs_required == 0 {
                invalid!("Required signatures cannot be 0".to_string())
            }

            // Cannot require more signatures than there are authorities
            if (new_authority_spec.sigs_required as usize) > new_authority_spec.authorized_signers.len() {
                invalid!(format!(
                    "There are not enough authorities ({}) to satisfy the signatures required ({})",
                    new_authority_spec.authorized_signers.len(),
                    new_authority_spec.sigs_required,
                ))
            }

            // There are no DeviceInviteAcceptance's in the chain
            if let Some(activity) = utils::get_latest_activity_for_entry_type(
                EntryTypesUnit::DeviceInviteAcceptance,
                &update.author,
                &update.prev_action,
            )? {
                invalid!(format!(
                    "Cannot change rules for KSR because a Device Invite was accepted at {} (action seq: {})",
                    activity.action.action().timestamp(),
                    activity.action.action().action_seq(),
                ))
            }

            // Get previous change rule
            let prev_change_rule = match utils::prev_change_rule( &update.author, &update.prev_action )? {
                Some(change_rule) => change_rule,
                None => invalid!(format!(
                    "No change rule found before action seq ({}) [{}]",
                    update.action_seq, update.prev_action
                )),
            };

            // Get authorized spec change and check signatures against previous authorities
            let sigs_required = &prev_change_rule.spec_change.new_spec.sigs_required;
            let authorities = &prev_change_rule.spec_change.new_spec.authorized_signers;
            let sig_count = change_rule_entry.spec_change.authorization_of_new_spec.len() as u8;

            if sig_count < *sigs_required {
                invalid!(format!(
                    "Signature count ({}) is not enough; change rule requires at least {} signatures",
                    sig_count, sigs_required,
                ))
            }

            utils::check_authorities(
                authorities,
                &change_rule_entry.spec_change.authorization_of_new_spec,
                &utils::serialize( &new_authority_spec )?,
            )?;

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
