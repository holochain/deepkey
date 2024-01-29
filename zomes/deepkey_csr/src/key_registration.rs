use crate::utils;
use deepkey::*;
use hdk::prelude::{
    *,
    holo_hash::DnaHash,
};
use hdk_extensions::{
    agent_id,
    must_get,
    hdi_extensions::{
        guest_error,
        trace_origin,
        ScopedTypeConnector,
    },
};

use crate::source_of_authority::query_keyset_authority_action_hash;


#[hdk_extern]
pub fn register_key(
    (new_key, new_key_signature, dna_hashes, app_name): (
        AgentPubKey,
        Signature,
        Vec<DnaHash>,
        String,
    ),
) -> ExternResult<ActionHash> {
    let derivation_details = DerivationDetails {
        app_index: crate::app_binding::query_app_bindings(())?.len() as u32,
        agent_index: 0,
    };

    // let key_type = KeyType::AppSig;

    // Create the KeyRegistration entry and its associated KeyAnchor entry directly after
    let key_registration = KeyRegistration::Create( (&new_key, &new_key_signature).into() );
    let key_registration_hash = create_entry( key_registration.to_input() )?;
    let key_anchor = KeyAnchor::from_agent_key( new_key.clone() )?;

    let key_anchor_addr = create_entry( key_anchor.to_input() )?;

    _create_new_key_entries(
        key_registration_hash.clone(),
        key_anchor,
        key_anchor_addr,
        dna_hashes,
        app_name,
        derivation_details,
        // key_type,
    )?;

    Ok(key_registration_hash)
}


#[hdk_extern]
pub fn update_key(
    (
        prior_key_registration,
        revocation_authorization,
        new_key,
        new_key_signature,
        dna_hashes,
        app_name,
    ): (
        ActionHash,
        Vec<Authorization>,
        AgentPubKey,
        Signature,
        Vec<DnaHash>,
        String,
    ),
) -> ExternResult<ActionHash> {
    // let derivation_index: u32 = 1;
    // let key_type = KeyType::AppSig;
    let history = trace_origin( &prior_key_registration )?;

    let derivation_details = DerivationDetails {
        app_index: 0, // Where can this come from?
        // - Derive previous anchor from previous registration
        // - Get record for previous anchor
        //   - We must be certain that there can only be 1 action for it
        // - Then query all key meta to find the one that points to this key anchor
        // - Use the same app_index
        agent_index: history.len() as u32,
    };

    let key_revocation = KeyRevocation::from(( &prior_key_registration, &revocation_authorization ));
    let key_generation = KeyGeneration::from(( &new_key, &new_key_signature ));

    // Create the KeyRegistration entry and its associated KeyAnchor entry directly after
    let key_registration = KeyRegistration::Update( key_revocation, key_generation );
    let key_registration_hash = update_entry(
        prior_key_registration.clone(),
        key_registration.to_input(),
    )?;

    let old_key_anchor_action_hash = _get_key_anchor_record_from_key_registration_action_hash(
        prior_key_registration.clone()
    )?;
    let key_anchor = KeyAnchor::from_agent_key( new_key.clone() )?;

    let key_anchor_addr = update_entry(
        old_key_anchor_action_hash,
        key_anchor.to_input(),
    )?;

    _create_new_key_entries(
        key_registration_hash.clone(),
        key_anchor,
        key_anchor_addr,
        dna_hashes,
        app_name,
        derivation_details,
        // key_type,
    )?;

    Ok(key_registration_hash)

    // TODO: Do we actually need to do this sort of thing when updating?
    // let old_agent_pubkey = match registration {
    //     KeyRegistration::Create(key_generation) => Ok(key_generation.new_key),
    //     KeyRegistration::CreateOnly(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
    //         "CreateOnly is Unimplemented"
    //     ))))?,
    //     KeyRegistration::Update(_, _) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
    //         "Cannot update: this key has already been revoked and updated to a new key."
    //     ))))?,
    //     KeyRegistration::Delete(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
    //         "Cannot update: Key is already revoked"
    //     )))),
    // }?;
}


#[hdk_extern]
pub fn revoke_key(
    (key_registration_to_revoke, revocation_authorization): (ActionHash, Vec<Authorization>),
) -> ExternResult<ActionHash> {
    let key_anchor_action_hash = _get_key_anchor_record_from_key_registration_action_hash(
        key_registration_to_revoke.clone(),
    )?;
    delete_entry(key_anchor_action_hash)?;

    let key_revocation = KeyRevocation {
        prior_key_registration: key_registration_to_revoke.clone(),
        revocation_authorization,
    };
    let revocation_registration = KeyRegistration::Delete(key_revocation);

    // TODO: Fill out the validation for KeyRevocation so it actually validates the revocation_authorization signatures.
    let key_registration_action_hash = update_entry(
        key_registration_to_revoke,
        revocation_registration.to_input(),
    )?;
    Ok(key_registration_action_hash)
}


fn _get_key_anchor_record_from_key_registration_action_hash(
    key_registration_to_revoke: ActionHash,
) -> ExternResult<ActionHash> {
    let record = must_get( &key_registration_to_revoke )?;

    let old_key_registration = KeyRegistration::try_from( record )?;
    let old_key_anchor = KeyAnchor::from_agent_key(
        _get_key_from_key_registration( old_key_registration )?
    )?;

    let record = must_get( &hash_entry( &old_key_anchor )? )?;

    Ok( record.action_address().clone() )
}


// All the entries in key creation:
fn _create_new_key_entries(
    key_registration_hash: ActionHash,
    key_anchor: KeyAnchor,
    key_anchor_addr: ActionHash,
    dna_hashes: Vec<DnaHash>,
    app_name: String,
    derivation_details: DerivationDetails,
    // key_type: KeyType,
) -> ExternResult<ActionHash> {
    // let derivation_path = format!("/dna/?/app/{app_name}");
    // Form of: [u8; 32]!
    // let derivation_path: [u8; 32] = hash_blake2b( Vec::from(derivation_path.as_bytes()), 32 )?
    //     .try_into()
    //     .map_err(|err| {
    //         guest_error!(format!("Error hashing derivation path. {:?}", err ))
    //     })?;

    // Create the private Metadata entries
    let key_meta = KeyMeta {
        key_anchor_addr,
        derivation_details,
        // derivation_bytes,
        // new_key: key_registration_hash.clone(),
        // key_type,
    };
    let key_meta_addr = create_entry( key_meta.to_input() )?;

    let app_binding = AppBinding {
        app_name,
        dna_hashes,
        key_meta_addr,
    };
    create_entry( app_binding.to_input() )?;

    // Create Link entry here: from KeySetRoot -> KeyAnchor
    let keyset_root_authority = query_keyset_authority_action_hash(())?;
    let key_anchor_hash = hash_entry( &key_anchor.to_input() )?;
    create_link(
        keyset_root_authority,
        key_anchor_hash.clone(),
        LinkTypes::KeysetRootToKeyAnchors,
        (),
    )?;

    create_link(
        agent_id()?,
        key_anchor_hash.clone(),
        LinkTypes::DeviceToKeyAnchor,
        (),
    )?;

    Ok(key_registration_hash)
}


fn _get_key_from_key_registration(key_registration: KeyRegistration) -> ExternResult<AgentPubKey> {
    let key = match key_registration.clone() {
        KeyRegistration::Create(key_generation) => key_generation.new_key,
        KeyRegistration::CreateOnly(key_generation) => key_generation.new_key,
        KeyRegistration::Update(_, key_generation) => key_generation.new_key,
        KeyRegistration::Delete(key_revocation) => {
            let old_keyreg_action = key_revocation.prior_key_registration;
            let old_keyreg = must_get( &old_keyreg_action )?;
            let key_registration = KeyRegistration::try_from(old_keyreg)?;
            match key_registration {
                KeyRegistration::Create(key_generation) => key_generation.new_key,
                KeyRegistration::CreateOnly(key_generation) => key_generation.new_key,
                KeyRegistration::Update(_, key_generation) => key_generation.new_key,
                KeyRegistration::Delete(_) => {
                    return Err(guest_error!(format!(
                        "Invalid KeyRevocation: attempted to revoke a revocation"
                    )))
                }
            }
        }
    };
    Ok(key)
}

// // TODO: Delete this function. For testing purposes only.
// #[hdk_extern]
// pub fn register_test_key(_: ()) -> ExternResult<KeyRegistration> {
//     // get this agent's pubkey
//     let agent_key = agent_info()?.agent_latest_pubkey;
//     let signature = sign(agent_key.clone(), agent_key.clone())?;
//     let key_registration = KeyRegistration::Create(KeyGeneration {
//         new_key: agent_key,
//         new_key_signing_of_author: signature,
//     });
//     new_key_registration(key_registration.clone())?;
//     Ok(key_registration)
// }


#[hdk_extern]
pub fn get_key_registration(
    original_key_registration_hash: ActionHash,
) -> ExternResult<KeyRegistration> {
    get_latest_key_registration(original_key_registration_hash)
}


fn get_latest_key_registration(key_registration_addr: ActionHash) -> ExternResult<KeyRegistration> {
    utils::get_latest_record( key_registration_addr )?.try_into()
}


// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateKeyRegistrationInput {
//     pub previous_key_registration_hash: ActionHash,
//     pub updated_key_registration: KeyRegistration,
// }
// #[hdk_extern]
// pub fn update_key_registration(
//     input: UpdateKeyRegistrationInput,
// ) -> ExternResult<Record> {
//     let updated_key_registration_hash = update_entry(
//         input.previous_key_registration_hash,
//         &input.updated_key_registration,
//     )?;
//     let record = get(updated_key_registration_hash.clone(), GetOptions::default())?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Could not find the newly updated KeyRegistration"))
//             ),
//         )?;
//     Ok(record)
// }
// #[hdk_extern]
// pub fn delete_key_registration(
//     original_key_registration_hash: ActionHash,
// ) -> ExternResult<ActionHash> {
//     delete_entry(original_key_registration_hash)
// }



// TODO: check if we need to do the `match registration` as in the old method below:
// #[hdk_extern]
// pub fn old_revoke_key(key_revocation: KeyRevocation) -> ExternResult<()> {
//     let registration = get(
//         key_revocation.prior_key_registration.clone(),
//         GetOptions::default(),
//     )?
//     .map(|record| record.entry().to_app_option::<KeyRegistration>())
//     .transpose()
//     .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
//     .flatten()
//     .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
//         "Cannot find the KeyRegistration to be revoked"
//     ))))?;
//     let agent_pubkey = match registration {
//         KeyRegistration::Create(key_generation) => Ok(key_generation.new_key),
//         KeyRegistration::CreateOnly(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
//             "CreateOnly is Unimplemented"
//         ))))?,
//         KeyRegistration::Update(_, _) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
//             "Cannot revoke: this key has already been revoked and updated to a new key."
//         ))))?,
//         KeyRegistration::Delete(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
//             "Cannot revoke: Key is already revoked"
//         )))),
//     }?;
//     let key_anchor = KeyAnchor::from_agent_key(agent_pubkey);
//     let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor.clone()))?;
//     let old_key_anchor_record =
//         get(key_anchor_hash, GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest(
//             String::from("Could not find the KeyAnchor for the KeyRegistration to be revoked")
//         )))?;

//     update_entry(
//         key_revocation.prior_key_registration.clone(),
//         &EntryTypes::KeyRegistration(KeyRegistration::Delete(key_revocation.clone())),
//     )?;
//     delete_entry(old_key_anchor_record.action_address().clone())?;
//     Ok(())
// }
