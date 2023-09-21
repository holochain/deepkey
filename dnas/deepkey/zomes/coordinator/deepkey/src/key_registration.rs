use deepkey_integrity::*;
use hdk::prelude::*;

use crate::source_of_authority::query_keyset_authority_action_hash;

#[hdk_extern]
// TODO: Only accept Create or CreateOnly
pub fn new_key_registration(key_registration: KeyRegistration) -> ExternResult<ActionHash> {
    // key anchor must be created from the key being registered
    // so we need to get the new key from the key registration
    let new_key = match key_registration.clone() {
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

    let key_anchor = KeyAnchor::from_agent_key(new_key);
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor.clone()))?;
    let keyset_root_authority = query_keyset_authority_action_hash(())?;

    // Create the KeyRegistration entry and its associated KeyAnchor entry
    let key_registration_hash = create_entry(EntryTypes::KeyRegistration(key_registration))?;
    create_entry(EntryTypes::KeyAnchor(key_anchor))?;

    // Create Link entry here: from KeySetRoot -> KeyAnchor
    create_link(
        keyset_root_authority,
        key_anchor_hash,
        LinkTypes::KeysetRootToKeyAnchors,
        (),
    )?;

    Ok(key_registration_hash)
}

// TODO: Delete this function. For testing purposes only.
#[hdk_extern]
pub fn register_test_key(_: ()) -> ExternResult<KeyRegistration> {
    // get this agent's pubkey
    let agent_key = agent_info()?.agent_latest_pubkey;
    let signature = sign(agent_key.clone(), agent_key.clone())?;
    let key_registration = KeyRegistration::Create(KeyGeneration {
        new_key: agent_key,
        new_key_signing_of_author: signature,
    });
    new_key_registration(key_registration.clone())?;
    Ok(key_registration)
}

#[hdk_extern]
pub fn update_key(
    (key_revocation, new_key_generation): (KeyRevocation, KeyGeneration),
) -> ExternResult<()> {
    let registration = get(
        key_revocation.prior_key_registration.clone(),
        GetOptions::default(),
    )?
    .map(|record| record.entry().to_app_option::<KeyRegistration>())
    .transpose()
    .map_err(|err| wasm_error!(WasmErrorInner::Guest(err.to_string())))?
    .flatten()
    .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
        "Cannot find the KeyRegistration to be revoked"
    ))))?;
    let old_agent_pubkey = match registration {
        KeyRegistration::Create(key_generation) => Ok(key_generation.new_key),
        KeyRegistration::CreateOnly(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "CreateOnly is Unimplemented"
        ))))?,
        KeyRegistration::Update(_, _) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Cannot update: this key has already been revoked and updated to a new key."
        ))))?,
        KeyRegistration::Delete(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Cannot update: Key is already revoked"
        )))),
    }?;
    let old_key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(
        KeyAnchor::from_agent_key(old_agent_pubkey).clone(),
    ))?;
    let old_key_anchor_record = get(old_key_anchor_hash, GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(
            "Could not find the KeyAnchor for the KeyRegistration to be revoked".into()
        )),
    )?;

    let new_key_anchor = KeyAnchor::from_agent_key(new_key_generation.new_key.clone());

    update_entry(
        key_revocation.prior_key_registration.clone(),
        &EntryTypes::KeyRegistration(KeyRegistration::Update(key_revocation, new_key_generation)),
    )?;
    update_entry(
        old_key_anchor_record.action_address().clone(),
        &EntryTypes::KeyAnchor(new_key_anchor),
    )?;
    Ok(())
}

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
