use deepkey_integrity::*;
use hdk::prelude::*;

// // function instantiate_key_revocation creates and returns a KeyRevocation in an ExternResult
// #[hdk_extern]
// pub fn instantiate_key_revocation(
//     prior_key_registration: ActionHash,
// ) -> ExternResult<KeyRevocation> {
//     Ok(KeyRevocation {
//         prior_key_registration,
//         revocation_authorization: Vec::new(),
//     })
// }

#[hdk_extern]
/// Receives a KeyRevocation, which in the simplest case could be a
/// prior_key_registration (ActionHash) and an empty Vec
/// Returns a KeyRevocation with an additional revocation_authorization signed by this agent
pub fn authorize_key_revocation(key_revocation: KeyRevocation) -> ExternResult<KeyRevocation> {
    let my_signature = sign(
        agent_info()?.agent_latest_pubkey,
        key_revocation.prior_key_registration.clone(),
    )?;
    // TODO: find index of my agent's key in authorized_signers
    // AuthoritySpec authorized_signers: Vec<AgentPubKey>

    let my_authorization = (0, my_signature); // TODO: replace 0 with index of my agent's key in authorized_signers
    let mut authorizations = key_revocation.revocation_authorization;
    authorizations.push(my_authorization);
    let new_key_revocation = KeyRevocation {
        prior_key_registration: key_revocation.prior_key_registration,
        revocation_authorization: authorizations,
    };
    Ok(new_key_revocation)
}

#[hdk_extern]
pub fn create_key_revocation_record(key_revocation: KeyRevocation) -> ExternResult<Record> {
    let key_revocation_hash = create_entry(&EntryTypes::KeyRevocation(key_revocation.clone()))?;
    let record = get(key_revocation_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created KeyRevocation"
        ))
    ))?;
    Ok(record)
}

#[hdk_extern]
pub fn get_key_revocation(
    original_key_revocation_hash: ActionHash,
) -> ExternResult<Option<Record>> {
    get_latest_key_revocation(original_key_revocation_hash)
}
fn get_latest_key_revocation(key_revocation_hash: ActionHash) -> ExternResult<Option<Record>> {
    let details = get_details(key_revocation_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest("KeyRevocation not found".into())
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
        Some(update) => get_latest_key_revocation(update.action_address().clone()),
        None => Ok(Some(record_details.record)),
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateKeyRevocationInput {
    pub previous_key_revocation_hash: ActionHash,
    pub updated_key_revocation: KeyRevocation,
}
#[hdk_extern]
pub fn update_key_revocation(input: UpdateKeyRevocationInput) -> ExternResult<Record> {
    let updated_key_revocation_hash = update_entry(
        input.previous_key_revocation_hash,
        &input.updated_key_revocation,
    )?;
    let record = get(updated_key_revocation_hash.clone(), GetOptions::default())?.ok_or(
        wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the newly updated KeyRevocation"
        ))),
    )?;
    Ok(record)
}
