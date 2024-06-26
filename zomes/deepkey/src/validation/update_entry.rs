use crate::{
    EntryTypes,
    EntryTypesUnit,

    KeyAnchor,
    KeyRegistration,
    KeyRevocation,
    ChangeRule,

    utils,

    validation::create_entry::{
        validate_key_generation,
    },
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_app_entry,

    // Macros
    guest_error,
    valid, invalid,
};


pub fn validation(
    app_entry: EntryTypes,
    update: Update,
    original_action_hash: ActionHash,
    _original_entry_hash: EntryHash
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(_) => {
            invalid!(format!("Keyset Roots cannot be updated"))
        },
        EntryTypes::ChangeRule(change_rule_entry) => {
            let base_change_rule_record = must_get_valid_record( original_action_hash.clone() )?;
            let base_change_rule_entry : ChangeRule = base_change_rule_record.try_into()?;

            // Keyset Root cannot be updated
            if base_change_rule_entry.keyset_root != change_rule_entry.keyset_root {
                invalid!(format!(
                    "The 'keyset_root' of this change rule cannot be changed; original entry 'keyset_root' is: {}",
                    base_change_rule_entry.keyset_root,
                ))
            }

            // Original action hash must be the ChangeRule create record
            //
            // NOTE: we cannot rely on 'original_action_hash' because we don't yet know how to
            // ensure it is the same chain.  Waiting to find out how 'author' equivalency will be
            // checked with agent updates.
            let create_change_rule_action = utils::base_change_rule(
                &update.author,
                &update.prev_action,
            )?;

            if create_change_rule_action.action_address() != &original_action_hash {
                invalid!(format!(
                    "The 'original_action_hash' is expected to be the ChangeRule create ({}); not '{}'",
                    create_change_rule_action.action_address(),
                    original_action_hash,
                ))
            }

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
            let prev_change_rule = match utils::prev_change_rule(
                &update.author,
                &update.prev_action,
            )? {
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
            let prior_key_reg_entry : KeyRegistration = summon_app_entry(
                &original_action_hash.into()
            )?;

            if let KeyRegistration::CreateOnly(_) = prior_key_reg_entry {
                invalid!(format!(
                    "Key registered using 'CreateOnly' cannot be updated"
                ))
            }

            match key_registration_entry {
                KeyRegistration::Create(..) |
                KeyRegistration::CreateOnly(..)=> {
                    invalid!(format!(
                        "KeyRegistration enum must be 'Update' or 'Delete'; not 'Create' or 'CreateOnly'"
                    ))
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
        EntryTypes::KeyAnchor(key_anchor_entry) => {
            // Check previous action is a key registration that matches this key anchor
            let key_reg : KeyRegistration = summon_app_entry( &update.prev_action.into() )?;

            let (key_rev, key_gen) = match key_reg {
                KeyRegistration::Update(key_rev, key_gen) => (key_rev, key_gen),
                _ => invalid!(format!(
                    "KeyAnchor update must be preceeded by a KeyRegistration::Update"
                )),
            };

            // Check new key
            if KeyAnchor::try_from( &key_gen.new_key )? != key_anchor_entry {
                invalid!(format!(
                    "KeyAnchor does not match KeyRegistration new key: {:#?} != {}",
                    key_anchor_entry, key_gen.new_key,
                ))
            }

            // Check revoked key - updated anchor must match the revoked registrations anchor
            let prior_key_anchor_entry : KeyAnchor = summon_app_entry( &original_action_hash.into() )?;
            let prior_key_reg : KeyRegistration = summon_app_entry(
                &key_rev.prior_key_registration.into()
            )?;

            if prior_key_reg.key_anchor()? != prior_key_anchor_entry {
                invalid!(format!(
                    "Original KeyAnchor does not match prior KeyRegistration key anchor: {:#?} != {:#?}",
                    prior_key_anchor_entry, prior_key_reg.key_anchor()?,
                ))
            }

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


pub fn validate_key_revocation(key_rev: &KeyRevocation, update: &Update) -> ExternResult<()> {
    // KeyRevocation {
    //     prior_key_registration: ActionHash,
    //     revocation_authorization: Vec<Authorization>,
    // }

    // Make sure the target key belongs to this KSR
    let key_registration_action = must_get_action( key_rev.prior_key_registration.to_owned() )?;

    if *key_registration_action.hashed.author() != update.author {
        Err(guest_error!(format!(
            "Author '{}' cannot revoke key registered by another author ({})",
            update.author, key_registration_action.hashed.author(),
        )))?
    }

    // Prevent duplicate updates to the same 'prior_key_registration'
    let activities = must_get_agent_activity(
        update.author.to_owned(),
        ChainFilter::new( update.prev_action.to_owned() )
            .until( key_rev.prior_key_registration.to_owned() )
            .include_cached_entries()
    )?;

    let entry_type = EntryType::try_from( EntryTypesUnit::KeyRegistration )?;
    let filtered_activities : Vec<RegisterAgentActivity> = activities.into_iter().filter(
        |activity| match activity.action.action().entry_type() {
            Some(et) => et == &entry_type,
            None => false,
        }
    ).collect();

    for activity in filtered_activities {
        let prior_key_reg : KeyRegistration = match activity.cached_entry {
            Some(entry) => entry,
            None => must_get_entry(
                activity.action.action().entry_hash().unwrap().to_owned()
            )?.content,
        }.try_into()?;

        let prior_key_rev = match prior_key_reg {
            KeyRegistration::Create(..) |
            KeyRegistration::CreateOnly(..)=> continue,
            KeyRegistration::Update( prior_key_rev, _ ) => prior_key_rev,
            KeyRegistration::Delete( prior_key_rev ) => prior_key_rev,
        };

        if prior_key_rev.prior_key_registration == key_rev.prior_key_registration {
            Err(guest_error!(format!(
                "There is already a KeyRegistration ({}) that revokes '{}'",
                activity.action.action_address(),
                key_rev.prior_key_registration,
            )))?
        }
    }

    // Get the current change rule
    let prev_change_rule = utils::prev_change_rule( &update.author, &update.prev_action )?
        .ok_or(guest_error!(format!(
            "No change rule found before action seq ({}) [{}]",
            update.action_seq, update.prev_action
        )))?;

    let sigs_required = &prev_change_rule.spec_change.new_spec.sigs_required;
    let authorities = &prev_change_rule.spec_change.new_spec.authorized_signers;
    let sig_count = key_rev.revocation_authorization.len() as u8;

    if sig_count < *sigs_required {
        Err(guest_error!(format!(
            "Signature count ({}) is not enough; key revocation requires at least {} signatures",
            sig_count, sigs_required,
        )))?
    }

    // Check authorizations against change rule authorities
    utils::check_authorities(
        authorities,
        &key_rev.revocation_authorization,
        &key_rev.prior_key_registration.clone().into_inner(),
    )?;

    Ok(())
}
