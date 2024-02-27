use crate::{
    EntryTypes,

    KeyAnchor,
    KeyRegistration,
    KeyGeneration,

    utils,
};

use hdi::prelude::*;
use hdi_extensions::{
    summon_app_entry,

    // Macros
    valid, invalid,
    guest_error,
};

// const KEYSET_ROOT_ACTION_SEQ : u32 = 3;
// const CHANGE_RULE_ACTION_SEQ : u32 = 4;


pub fn validation(
    app_entry: EntryTypes,
    create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(ksr_entry) => {
            // Check action seq
            // if create.action_seq != KEYSET_ROOT_ACTION_SEQ {
            //     invalid!(format!(
            //         "KeysetRoot has invalid chain index ({}); must be chain index {}",
            //         create.action_seq, KEYSET_ROOT_ACTION_SEQ,
            //     ));
            // }

            // Check signature matches root pub key
            if verify_signature_raw(
                ksr_entry.root_pub_key_as_agent(),
                ksr_entry.signed_fda,
                ksr_entry.first_deepkey_agent.clone().into_inner()
            )? == false {
                invalid!("KeysetRoot has invalid signature".to_string());
            }

            // Check that FDA is the chain author
            if create.author != ksr_entry.first_deepkey_agent {
                invalid!(format!(
                    "KeysetRoot expected FDA to be '{}', not '{}'; FDA must be the action author",
                    create.author, ksr_entry.first_deepkey_agent,
                ));
            }

            valid!()
        },
        EntryTypes::ChangeRule(change_rule_entry) => {
            // Check action seq
            // if create.action_seq != CHANGE_RULE_ACTION_SEQ {
            //     invalid!(format!(
            //         "ChangeRule has invalid chain index ({}); must be chain index {}",
            //         create.action_seq, CHANGE_RULE_ACTION_SEQ,
            //     ));
            // }

            // KeysetRoot originates in this chain (perhaps it should also be the previous action)
            let (signed_action, _) = utils::get_keyset_root(
                &create.author,
                &create.prev_action,
            )?;

            if *signed_action.action_address() != change_rule_entry.keyset_root {
                invalid!(format!(
                    "Change rule keyset root ({}) does not belong to this chain's KSR '{}'",
                    change_rule_entry.keyset_root,
                    signed_action.action_address(),
                ))
            }

            // There can only be 1 'Create' ChangeRule per KSR
            if let Some(_) = utils::prev_change_rule( &create.author, &create.prev_action )? {
                invalid!(format!(
                    "There is already a change rule for KeysetRoot '{}'",
                    change_rule_entry.keyset_root,
                ))
            }

            valid!()
        },
        EntryTypes::KeyRegistration(key_registration_entry) => {
            match key_registration_entry {
                KeyRegistration::Create( key_gen ) => {
                    validate_key_generation( &key_gen, &create.into() )?;

                    valid!()
                },
                KeyRegistration::CreateOnly( key_gen ) => {
                    validate_key_generation( &key_gen, &create.into() )?;

                    valid!()
                },
                KeyRegistration::Update(..) |
                KeyRegistration::Delete(..) => {
                    invalid!("KeyRegistration enum must be 'Create' or 'CreateOnly'; not 'Update' or 'Delete'".to_string())
                },
            }
        },
        EntryTypes::KeyAnchor(key_anchor_entry) => {
            // Check previous action is a key registration that matches this key anchor
            let key_reg : KeyRegistration = summon_app_entry( &create.prev_action.into() )?;

            let key_gen = match key_reg {
                KeyRegistration::Create(key_gen) => key_gen,
                KeyRegistration::CreateOnly(key_gen) => key_gen,
                _ => invalid!(format!("KeyAnchor create must be preceeded by a KeyRegistration::[Create | CreateOnly]")),
            };

            if KeyAnchor::try_from( &key_gen.new_key )? != key_anchor_entry {
                invalid!(format!(
                    "KeyAnchor does not match KeyRegistration new key: {:#?} != {}",
                    key_anchor_entry, key_gen.new_key,
                ))
            }

            valid!()
        },

        // Private Records
        EntryTypes::KeyMeta(_key_meta_entry) => {
            valid!()
        },
        EntryTypes::AppBinding(_app_binding_entry) => {
            valid!()
        },

        // _ => invalid!(format!("Create validation not implemented for entry type: {:#?}", create.entry_type )),
    }
}


pub fn validate_key_generation(key_gen: &KeyGeneration, creation: &EntryCreationAction) -> ExternResult<()> {
    // KeyGeneration {
    //     new_key: AgentPubKey,
    //     new_key_signing_of_author: Signature,
    // }

    // Signature matches author
    if verify_signature_raw(
        key_gen.new_key.to_owned(),
        key_gen.new_key_signing_of_author.to_owned(),
        creation.author().get_raw_39().to_vec(),
    )? == false {
        Err(guest_error!(format!(
            "Signature does not match new key ({})",
            key_gen.new_key,
        )))?;
    }

    Ok(())
}
