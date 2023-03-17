// use hdk::prelude::*;
// use deepkey_integrity::*;
// #[hdk_extern]
// pub fn create_key_registration(
//     key_registration: KeyRegistration,
// ) -> ExternResult<Record> {
//     let key_registration_hash = create_entry(
//         &EntryTypes::KeyRegistration(key_registration.clone()),
//     )?;
//     let record = get(key_registration_hash.clone(), GetOptions::default())?
//         .ok_or(
//             wasm_error!(
//                 WasmErrorInner::Guest(String::from("Could not find the newly created KeyRegistration"))
//             ),
//         )?;
//     Ok(record)
// }
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
