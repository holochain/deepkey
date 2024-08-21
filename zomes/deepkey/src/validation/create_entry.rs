use crate::{
    EntryTypes,

    KeysetRoot,
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

const KEYSET_ROOT_ACTION_SEQ : u32 = 3;
const CHANGE_RULE_ACTION_SEQ : u32 = 4;


pub fn validation(
    app_entry: EntryTypes,
    create: Create
) -> ExternResult<ValidateCallbackResult> {
    match app_entry {
        EntryTypes::KeysetRoot(ksr_entry) => {
            // Check action seq
            if create.action_seq != KEYSET_ROOT_ACTION_SEQ {
                invalid!(format!(
                    "KeysetRoot has invalid chain index ({}); must be chain index {}",
                    create.action_seq, KEYSET_ROOT_ACTION_SEQ,
                ));
            }

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
            if create.action_seq != CHANGE_RULE_ACTION_SEQ {
                invalid!(format!(
                    "ChangeRule has invalid chain index ({}); must be chain index {}",
                    create.action_seq, CHANGE_RULE_ACTION_SEQ,
                ));
            }

            // KeysetRoot originates in this chain (perhaps it should also be the previous action)
            if create.prev_action != change_rule_entry.keyset_root {
                invalid!(format!(
                    "Change rule keyset root ({}) does not belong to this chain's KSR '{}'",
                    change_rule_entry.keyset_root,
                    create.prev_action,
                ))
            }

            // No signature checks are required because the author of the change rule will always
            // match the KSR's FDA.  Therefore the internal Holochain validation ensures that the
            // action could only be created by the current change rule authority (ie. the FDA).

            // The new spec must be 1 of 1 with the FDA as the authority
            let ksr_record = must_get_valid_record( create.prev_action )?;
            let ksr : KeysetRoot = ksr_record.try_into()?;
            let fda_key_bytes = utils::keybytes_from_agentpubkey( &ksr.first_deepkey_agent )?;
            let new_spec = change_rule_entry.spec_change.new_spec;

            if new_spec.sigs_required != 1 {
                invalid!(format!(
                    "The initial change rule must require 1 signature; not '{}'",
                    new_spec.sigs_required,
                ))
            }

            if new_spec.authorized_signers.len() != 1 {
                invalid!(format!(
                    "The initial change rule must have 1 authority that is the FDA; not '{}'",
                    new_spec.authorized_signers.len(),
                ))
            }

            if new_spec.authorized_signers[0] != fda_key_bytes {
                invalid!(format!(
                    "The initial change rule must have 1 authority that is the FDA ({}); not '{}'",
                    ksr.first_deepkey_agent,
                    AgentPubKey::from_raw_32( new_spec.authorized_signers[0].to_vec() ),
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
