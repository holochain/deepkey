use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
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

    let key_registration_hash = create_entry(EntryTypes::KeyRegistration(key_registration))?;
    create_entry(EntryTypes::KeyAnchor(KeyAnchor::from_agent_key(new_key)))?;
    Ok(key_registration_hash)
}

#[hdk_extern]
pub fn get_key_registration_from_key_anchor(
    agent_pubkey: AgentPubKey,
) -> ExternResult<Option<Record>> {
    let key_anchor = KeyAnchor::from_agent_key(agent_pubkey);
    let key_anchor_clone = key_anchor.clone(); // Clone the key_anchor
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor_clone))?;
    let key_registration_record = get(key_anchor_hash, GetOptions::default())?
        .map(|key_anchor_element| {
            key_anchor_element.action().prev_action().map(|action| action.to_owned())
        })
        .flatten()
        .map(|key_registration_action| {
            get(key_registration_action.clone(), GetOptions::default())
        })
        .transpose()?
        .flatten();
    Ok(key_registration_record)
}



// #[hdk_extern]
// pub fn get_key_registration(
//     original_key_registration_hash: ActionHash,
// ) -> ExternResult<Option<Record>> {
//     get_latest_key_registration(original_key_registration_hash)
// }
// fn get_latest_key_registration(
//     key_registration_hash: ActionHash,
// ) -> ExternResult<Option<Record>> {
//     let details = get_details(key_registration_hash, GetOptions::default())?
//         .ok_or(wasm_error!(WasmErrorInner::Guest("KeyRegistration not found".into())))?;
//     let record_details = match details {
//         Details::Entry(_) => {
//             Err(wasm_error!(WasmErrorInner::Guest("Malformed details".into())))
//         }
//         Details::Record(record_details) => Ok(record_details),
//     }?;
//     if record_details.deletes.len() > 0 {
//         return Ok(None);
//     }
//     match record_details.updates.last() {
//         Some(update) => get_latest_key_registration(update.action_address().clone()),
//         None => Ok(Some(record_details.record)),
//     }
// }
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
