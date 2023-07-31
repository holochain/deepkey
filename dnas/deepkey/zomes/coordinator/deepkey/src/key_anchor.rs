use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_entry_helper]
pub enum KeyState {
    NotFound,
    Invalidated(SignedActionHashed),
    Valid(SignedActionHashed),
}

#[hdk_extern]
pub fn key_state((agent_key, _timestamp): (AgentPubKey, Timestamp)) -> ExternResult<KeyState> {
    // let key_anchor_bytes = key_anchor_vec.try_into().map_err(|_| {
    //     wasm_error!(WasmErrorInner::Guest(
    //         "Could not convert key_anchor_vec to [u8; 32]".into()
    //     ))
    // })?;
    // let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(KeyAnchor {
    //     bytes: key_anchor_bytes,
    // }))?;
    // TODO: should pass in the actual key anchor, but can't deserialize [u8; 32] from JS.
    // Reference: https://docs.rs/serde_with/1.10.0/serde_with/struct.Bytes.html
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(KeyAnchor::from_agent_key(agent_key)))?;

    // find any deletes, anything pointing to this key anchor
    let key_anchor_details_opt = get_details(key_anchor_hash.clone(), GetOptions::default())?;
    if let None = key_anchor_details_opt {
        return Ok(KeyState::NotFound);
    }
    let key_anchor_details = key_anchor_details_opt.unwrap();
    match key_anchor_details {
        Details::Entry(entry_details) => entry_details
            .deletes
            .first()
            .or(entry_details.updates.first())
            // TODO: use timestamp on the update or delete header to determine if it's invalid before the timestamp
            // get earliest timestamp, in case multiple updates/deletes happened at different time to the same key
            .map(|action| Ok(KeyState::Invalidated(action.clone())))
            .or(entry_details
                .actions
                .first()
                .map(|action| Ok(KeyState::Valid(action.clone()))))
            .unwrap_or(Ok(KeyState::NotFound)),
        Details::Record(_) => Err(wasm_error!(WasmErrorInner::Guest(
            "Problem with KeyAnchor record".into()
        )))?,
    }
}

#[hdk_extern]
// Note, we're still passing in an agent_pubkey; at some point we will want to have methods that
// take in a KeyAnchor directly.
pub fn get_key_registration_from_agent_pubkey_key_anchor(
    agent_pubkey: AgentPubKey,
) -> ExternResult<Option<Record>> {
    let key_anchor = KeyAnchor::from_agent_key(agent_pubkey);
    let key_anchor_clone = key_anchor.clone(); // Clone the key_anchor
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor_clone))?;
    let key_registration_record = get(key_anchor_hash, GetOptions::default())?
        .map(|key_anchor_element| {
            key_anchor_element
                .action()
                .prev_action()
                .map(|action| action.to_owned())
        })
        .flatten()
        .map(|key_registration_action| get(key_registration_action.clone(), GetOptions::default()))
        .transpose()?
        .flatten();
    Ok(key_registration_record)
}

#[hdk_extern]
pub fn get_agent_pubkey_key_anchor(agent_pubkey: AgentPubKey) -> ExternResult<Option<Record>> {
    let key_anchor = KeyAnchor::from_agent_key(agent_pubkey);
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor.clone()))?;
    get(key_anchor_hash, GetOptions::default())
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
