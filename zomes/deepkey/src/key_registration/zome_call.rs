use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::key_registration::entry::KeyRegistration;
use deepkey_integrity::key_registration::error::Error;
use deepkey_integrity::key_anchor::entry::KeyAnchor;
use deepkey_integrity::key_registration::entry::KeyRevocation;

fn revoked_anchor_element(key_revocation: &KeyRevocation) -> ExternResult<Record> {
    let old_key_registration_element = match get(key_revocation.as_prior_key_registration_ref().clone(), GetOptions::content())? {
        Some(old_key_registration_element) => old_key_registration_element,
        None => return Err(Error::UpdatedKeyRegistrationLookup.into()),
    };
    let old_key_anchor_seq = old_key_registration_element.action().action_seq() + 1;
    let query = ChainQueryFilter::new().sequence_range(ChainQueryFilterRange::ActionSeqRange(old_key_anchor_seq, old_key_anchor_seq+1));
    let agent_activty = get_agent_activity(
        old_key_registration_element.action().author().to_owned(),
        query,
        ActivityRequest::Full,
    )?;
    match agent_activty.status {
        ChainStatus::Valid(_) => if agent_activty.valid_activity.len() == 1 {
            match get(agent_activty.valid_activity[0].1.clone(), GetOptions::content())? {
                Some(old_key_anchor_element) => Ok(old_key_anchor_element),
                None => Err(Error::UpdatedKeyAnchorLookup.into()),
            }
        } else {
            Err(Error::UpdatedKeyAnchorLookup.into())
        },
        _ => Err(Error::UpdatedKeyAnchorLookup.into()),
    }
}

#[hdk_extern]
/// Returns the key anchor action hash.
fn new_key_registration(key_registration: KeyRegistration) -> ExternResult<ActionHash> {
    match &key_registration {
        KeyRegistration::Create(key_generation) | KeyRegistration::CreateOnly(key_generation) => {
            let key_anchor = KeyAnchor::from(key_generation);
            create_entry(key_registration)?;
            create_entry(key_anchor)
        },
        KeyRegistration::Update(key_revocation, key_generation) => {
            let new_key_anchor = KeyAnchor::from(key_generation);
            let old_key_anchor_element = revoked_anchor_element(&key_revocation)?;
            update_entry(key_revocation.as_prior_key_registration_ref().clone(), key_registration)?;
            update_entry(old_key_anchor_element.action_hashed().as_hash().to_owned(), new_key_anchor)
        },
        KeyRegistration::Delete(key_revocation) => {
            let old_key_anchor_element = revoked_anchor_element(&key_revocation)?;
            let update_delete = update_entry(key_revocation.as_prior_key_registration_ref().clone(), key_registration)?;
            delete_entry(update_delete)?;
            delete_entry(old_key_anchor_element.action_hashed().as_hash().to_owned())
        }
    }
}
