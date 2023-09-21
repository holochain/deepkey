use deepkey_integrity::*;
use hdk::prelude::{holo_hash::hash_type::Dna, *};

use crate::source_of_authority::query_keyset_authority_action_hash;

#[hdk_extern]
pub fn register_key(
    // TODO: Create the AgentPubKey in Lair using the derivation_path etc instead of passing it in here.
    (new_key, new_key_signature, dna_hash, app_name): (
        AgentPubKey,
        Signature,
        HoloHash<Dna>,
        String,
    ),
) -> ExternResult<ActionHash> {
    let derivation_index: u32 = 1;
    // TODO: Figure out when to set different key_types and what they mean
    let key_type = KeyType::AppSig;

    // Create the KeyRegistration entry and its associated KeyAnchor entry directly after
    let key_registration = KeyRegistration::Create(KeyGeneration {
        new_key: new_key.clone(),
        new_key_signing_of_author: new_key_signature,
    });
    let key_registration_hash = create_entry(EntryTypes::KeyRegistration(key_registration))?;
    let key_anchor = KeyAnchor::from_agent_key(new_key.clone());
    create_entry(EntryTypes::KeyAnchor(key_anchor.clone()))?;

    _create_new_key_entries(
        key_registration_hash.clone(),
        key_anchor,
        dna_hash,
        app_name,
        derivation_index,
        key_type,
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
        dna_hash,
        app_name,
    ): (
        ActionHash,
        Vec<Authorization>,
        AgentPubKey,
        Signature,
        HoloHash<Dna>,
        String,
    ),
) -> ExternResult<ActionHash> {
    // TODO: We should be traversing all the previous keys and counting up to calculate the derivation index.
    let derivation_index: u32 = 1;
    let key_type = KeyType::AppSig;

    let key_revocation = KeyRevocation {
        prior_key_registration: prior_key_registration.clone(),
        revocation_authorization,
    };

    let key_generation = KeyGeneration {
        new_key: new_key.clone(),
        new_key_signing_of_author: new_key_signature,
    };
    // Create the KeyRegistration entry and its associated KeyAnchor entry directly after
    let key_registration = KeyRegistration::Update(key_revocation, key_generation);
    let key_registration_hash = update_entry(
        prior_key_registration.clone(),
        EntryTypes::KeyRegistration(key_registration),
    )?;

    let old_key_anchor_action_hash =
        _get_key_anchor_record_from_key_registration_action_hash(prior_key_registration.clone())?;
    let key_anchor = KeyAnchor::from_agent_key(new_key.clone());
    update_entry(
        old_key_anchor_action_hash,
        EntryTypes::KeyAnchor(key_anchor.clone()),
    )?;

    _create_new_key_entries(
        key_registration_hash.clone(),
        key_anchor,
        dna_hash,
        app_name,
        derivation_index,
        key_type,
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
        EntryTypes::KeyRegistration(revocation_registration),
    )?;
    Ok(key_registration_action_hash)
}

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

fn _get_key_anchor_record_from_key_registration_action_hash(
    key_registration_to_revoke: ActionHash,
) -> ExternResult<ActionHash> {
    let record =
        get(key_registration_to_revoke.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Cannot query old key registration to revoke."))
        ))?;
    let old_key_registration = record
        .entry
        .to_app_option::<KeyRegistration>()
        .map_err(|err| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Could not deserialize old key registration to revoke. {:?}",
                err
            )))
        })?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Provided old key registration does not exist"
        ))))?;

    let old_key_anchor =
        KeyAnchor::from_agent_key(_get_key_from_key_registration(old_key_registration)?);

    let record = get(hash_entry(old_key_anchor)?, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Old Key anchor does not exist on the chain."))
    ))?;

    Ok(record.action_address().clone())
}

// All the entries in key creation:
fn _create_new_key_entries(
    key_registration_hash: ActionHash,
    key_anchor: KeyAnchor,
    dna_hash: HoloHash<Dna>,
    app_name: String,
    derivation_index: u32,
    key_type: KeyType,
) -> ExternResult<ActionHash> {
    let derivation_path = format!("/dna/{dna_hash}/app/{app_name}");
    // Form of: [u8; 32]!
    let derivation_path: [u8; 32] = hash_blake2b(Vec::from(derivation_path.as_bytes()), 32)?
        .try_into()
        .map_err(|err| {
            wasm_error!(WasmErrorInner::Guest(format!(
                "Error hashing derivation path. {:?}",
                err
            )))
        })?;

    // Create the private Metadata entries
    let key_meta = KeyMeta {
        derivation_index,
        derivation_path,
        new_key: key_registration_hash.clone(),
        key_type,
    };
    let key_meta_hash = create_entry(EntryTypes::KeyMeta(key_meta))?;

    let dna_binding = DnaBinding {
        app_name,
        dna_hash,
        key_meta: key_meta_hash,
    };
    create_entry(EntryTypes::DnaBinding(dna_binding))?;

    // Create Link entry here: from KeySetRoot -> KeyAnchor
    let keyset_root_authority = query_keyset_authority_action_hash(())?;
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor))?;
    create_link(
        keyset_root_authority,
        key_anchor_hash,
        LinkTypes::KeysetRootToKeyAnchors,
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
            let old_keyreg = get(old_keyreg_action, GetOptions::default())?.ok_or(wasm_error!(
                WasmErrorInner::Guest("KeyRegistration not found".into())
            ))?;
            let key_registration = KeyRegistration::try_from(old_keyreg)?;
            match key_registration {
                KeyRegistration::Create(key_generation) => key_generation.new_key,
                KeyRegistration::CreateOnly(key_generation) => key_generation.new_key,
                KeyRegistration::Update(_, key_generation) => key_generation.new_key,
                KeyRegistration::Delete(_) => {
                    return Err(wasm_error!(WasmErrorInner::Guest(
                        "Invalid KeyRevocation: attempted to revoke a revocation".into()
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
) -> ExternResult<Option<Record>> {
    get_latest_key_registration(original_key_registration_hash)
}
fn get_latest_key_registration(key_registration_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(key_registration_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("KeyRegistration not found".into())
    ))?;
    let record_details = match details {
        Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Malformed details".into()
        ))),
        Details::Record(record_details) => Ok(record_details),
    }?;
    if record_details.deletes.len() > 0 {
        return Ok(None);
    }
    match record_details.updates.last() {
        Some(update) => get_latest_key_registration(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
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
