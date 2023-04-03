use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_entry_helper]
pub enum KeyState {
    NotFound,
    Invalidated,
    Valid,
}

pub fn key_state((key_anchor_bytes, _timestamp): ([u8; 32], Timestamp)) -> ExternResult<KeyState> {
    #[allow(unused_variables)]
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(KeyAnchor {
        bytes: key_anchor_bytes,
    }))?;

    // TODO: get details here
    // find any deletes, anything pointing to this key anchor
    // let details_opt = get_details(key_anchor_hash, GetOptions::default())?;

    // let key_anchor_opt = get(key_anchor_hash.clone(), GetOptions::default())?;
    // if let None = key_anchor_opt {
    //     return Ok(KeyState::NotFound);
    // }
    // let key_anchor = key_anchor_opt.unwrap();

    // let key_registration_actionhash_opt = key_anchor.action().prev_action();
    // if let None = key_registration_actionhash_opt {
    //     return Ok(KeyState::NotFound);
    // }
    // let key_registration_actionhash = key_registration_actionhash_opt.unwrap().clone();

    // let key_registration_record_opt = get(key_registration_actionhash, GetOptions::default())?;
    // if let None = key_registration_record_opt {
    //     return Ok(KeyState::NotFound);
    // }
    // let key_registration_record = key_registration_record_opt.unwrap().clone();

    // let key_registration_opt = key_registration_record.entry.into_option();
    // if let None = key_registration_opt {
    //     return Ok(KeyState::NotFound);
    // }
    // let key_registration_entry = key_registration_opt.unwrap().clone();

    // let key_registration = KeyRegistration::try_from(key_registration_entry)?;

    Ok(KeyState::Valid)
}

// #[hdk_extern]
// pub fn create_key_anchor(key_anchor: KeyAnchor) -> ExternResult<Record> {
//     let key_anchor_hash = create_entry(&EntryTypes::KeyAnchor(key_anchor.clone()))?;
//     let record = get(key_anchor_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
//         WasmErrorInner::Guest(String::from("Could not find the newly created KeyAnchor"))
//     ))?;
//     Ok(record)
// }
// #[hdk_extern]
// pub fn get_key_anchor(original_key_anchor_hash: ActionHash) -> ExternResult<Option<Record>> {
//     get_latest_key_anchor(original_key_anchor_hash)
// }
// fn get_latest_key_anchor(key_anchor_hash: ActionHash) -> ExternResult<Option<Record>> {
//     let details = get_details(key_anchor_hash, GetOptions::default())?.ok_or(wasm_error!(
//         WasmErrorInner::Guest("KeyAnchor not found".into())
//     ))?;
//     let record_details = match details {
//         Details::Entry(_) => Err(wasm_error!(WasmErrorInner::Guest(
//             "Malformed details".into()
//         ))),
//         Details::Record(record_details) => Ok(record_details),
//     }?;
//     if record_details.deletes.len() > 0 {
//         return Ok(None);
//     }
//     match record_details.updates.last() {
//         Some(update) => get_latest_key_anchor(update.action_address().clone()),
//         None => Ok(Some(record_details.record)),
//     }
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateKeyAnchorInput {
//     pub previous_key_anchor_hash: ActionHash,
//     pub updated_key_anchor: KeyAnchor,
// }
// #[hdk_extern]
// pub fn update_key_anchor(input: UpdateKeyAnchorInput) -> ExternResult<Record> {
//     let updated_key_anchor_hash =
//         update_entry(input.previous_key_anchor_hash, &input.updated_key_anchor)?;
//     let record =
//         get(updated_key_anchor_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
//             WasmErrorInner::Guest(String::from("Could not find the newly updated KeyAnchor"))
//         ))?;
//     Ok(record)
// }
// #[hdk_extern]
// pub fn delete_key_anchor(original_key_anchor_hash: ActionHash) -> ExternResult<ActionHash> {
//     delete_entry(original_key_anchor_hash)
// }
