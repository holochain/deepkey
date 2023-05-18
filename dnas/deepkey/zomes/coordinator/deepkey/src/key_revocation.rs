use deepkey_integrity::*;
use hdk::prelude::*;

// function instantiate_key_revocation creates and returns a KeyRevocation in an ExternResult
#[hdk_extern]
pub fn instantiate_key_revocation(
    prior_key_registration: ActionHash,
) -> ExternResult<KeyRevocation> {
    Ok(KeyRevocation {
        prior_key_registration,
        revocation_authorization: Vec::new(),
    })
}

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
pub fn revoke_key(key_revocation: KeyRevocation) -> ExternResult<()> {
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
    let agent_pubkey = match registration {
        KeyRegistration::Create(key_generation) => Ok(key_generation.new_key),
        KeyRegistration::CreateOnly(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "CreateOnly is Unimplemented"
        ))))?,
        KeyRegistration::Update(_, _) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Cannot revoke: this key has already been revoked and updated to a new key."
        ))))?,
        KeyRegistration::Delete(_) => Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Cannot revoke: Key is already revoked"
        )))),
    }?;
    let key_anchor = KeyAnchor::from_agent_key(agent_pubkey);
    let key_anchor_hash = hash_entry(&EntryTypes::KeyAnchor(key_anchor.clone()))?;
    let old_key_anchor_record =
        get(key_anchor_hash, GetOptions::default())?.ok_or(wasm_error!(WasmErrorInner::Guest(
            String::from("Could not find the KeyAnchor for the KeyRegistration to be revoked")
        )))?;

    update_entry(
        key_revocation.prior_key_registration.clone(),
        &EntryTypes::KeyRegistration(KeyRegistration::Delete(key_revocation.clone())),
    )?;
    delete_entry(old_key_anchor_record.action_address().clone())?;
    Ok(())
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
