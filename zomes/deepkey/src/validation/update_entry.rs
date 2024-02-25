use crate::{
    EntryTypes,

    KeyRegistration,
    KeyRevocation,

    utils,

    validation::create_entry::{
        validate_key_generation,
    },
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
        EntryTypes::KeyRegistration(key_registration_entry) => {
            match key_registration_entry {
                KeyRegistration::Create(..) |
                KeyRegistration::CreateOnly(..)=> {
                    invalid!("KeyRegistration enum must be 'Update' or 'Delete'; not 'Create' or 'CreateOnly'".to_string())
                },
                KeyRegistration::Update( key_rev, key_gen ) => {
                    validate_key_revocation( &key_rev, &update )?;
                    validate_key_generation( &key_gen, &update.into() )?;

                    valid!()
                 },
                KeyRegistration::Delete( key_rev ) => {
                    validate_key_revocation( &key_rev, &update )?;

                    valid!()
                },
            }
        },
        EntryTypes::KeyAnchor(_key_anchor_entry) => {
            valid!()
        },
        EntryTypes::KeyMeta(_key_meta_entry) => {
            invalid!(format!("Key Meta cannot be updated"))
        },
        EntryTypes::AppBinding(_app_binding_entry) => {
            invalid!(format!("App Binding cannot be updated"))
        },
        // _ => invalid!(format!("Update validation not implemented for entry type: {:#?}", update.entry_type )),
    }
}


pub fn validate_key_revocation(_key_rev: &KeyRevocation, _update: &Update) -> ExternResult<()> {
    // KeyRevocation {
    //     prior_key_registration: ActionHash,
    //     revocation_authorization: Vec<Authorization>,
    // }

    // Trace the KSR this key is under at this time

    // Get the current change rule

    // Check authorizations against change rule authorities

    Ok(())
}
