use crate::{
    EntryTypesUnit,

    KeysetRoot,
    ChangeRule,

    KeyBytes,
    Authorization,
};
use rmp_serde;
use hdi_extensions::{
    guest_error,
};
use hdi::prelude::*;
use hdk::prelude::debug;


pub fn serialize<T>(target: &T) -> ExternResult<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    rmp_serde::encode::to_vec( target )
        .map_err( |err| guest_error!(format!(
            "Failed to serialize target: {:?}", err
        )) )
}


pub fn get_activities_for_entry_type<T,E>(
    entry_type_unit: T,
    author: &AgentPubKey,
    chain_top: &ActionHash,
) -> ExternResult<Vec<RegisterAgentActivity>>
where
    T: Copy,
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    debug!("Getting agent activity for {} (chain top: {})", author, chain_top );
    let activities = must_get_agent_activity(
        author.to_owned(),
        ChainFilter::new(chain_top.to_owned())
            .include_cached_entries()
    )?;

    debug!("Found {} activities", activities.len() );
    let entry_type = EntryType::try_from( entry_type_unit )?;

    let filtered_activities : Vec<RegisterAgentActivity> = activities.into_iter().filter(
        |activity| match activity.action.action().entry_type() {
            Some(et) => et == &entry_type,
            None => false,
        }
    ).collect();
    debug!("Found {} activities for entry type: {}", filtered_activities.len(), entry_type );

    Ok( filtered_activities )
}


pub fn get_latest_activity_for_entry_type<T,E>(
    entry_type_unit: T,
    author: &AgentPubKey,
    chain_top: &ActionHash,
) -> ExternResult<Option<RegisterAgentActivity>>
where
    T: Copy,
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    let activities = get_activities_for_entry_type(
        entry_type_unit,
        author,
        chain_top,
    )?;

    Ok(activities.first().cloned())
}


pub fn get_keyset_root(
    author: &AgentPubKey,
    chain_top: &ActionHash,
) -> ExternResult<(SignedActionHashed, KeysetRoot)> {
    let keyset_root_activity = get_latest_activity_for_entry_type(
        EntryTypesUnit::KeysetRoot,
        &author,
        &chain_top,
    )?;

    if let Some(activity) = keyset_root_activity {
        let entry_hash = activity.action.action().entry_hash()
            .ok_or(guest_error!(format!(
                "Expected action (seq: {}) to have an entry hash",
                activity.action.action().action_seq()
            )))?;

        Ok((
            activity.action.clone(),
            must_get_entry( entry_hash.to_owned() )?.try_into()?
        ))
    }
    else {
        Err(guest_error!(format!("Author ({}) chain is missing a KeysetRoot", author )))
    }
}


pub fn prev_change_rule (
    author: &AgentPubKey,
    chain_top: &ActionHash
) -> ExternResult<Option<ChangeRule>> {
    let latest_change_rule = get_latest_activity_for_entry_type(
        EntryTypesUnit::ChangeRule,
        author,
        chain_top,
    )?;

    debug!("Latest ChangeRule activity: {:?}", latest_change_rule );
    Ok(
        match latest_change_rule {
            Some(activity) => {
                debug!("ChangeRule cached entry: {:?}", activity.cached_entry );
                Some(match activity.cached_entry {
                    Some(entry) => entry,
                    None => must_get_entry(
                        activity.action.action().entry_hash().unwrap().to_owned()
                    )?.content,
                }.try_into()?)
            },
            None => None,
        }
    )
}


pub fn base_change_rule (
    author: &AgentPubKey,
    chain_top: &ActionHash
) -> ExternResult<SignedActionHashed> {
    let change_rules = get_activities_for_entry_type(
        EntryTypesUnit::ChangeRule,
        author,
        chain_top,
    )?;

    let filtered_activities : Vec<RegisterAgentActivity> = change_rules.into_iter().filter(
        |activity| activity.action.action().action_type() == ActionType::Create
    ).collect();

    Ok(
        filtered_activities.first()
            .ok_or(guest_error!(format!(
                "There is no ChangeRule create action on source chain ({}) with chain type: {}",
                author, chain_top,
            )))?
            .to_owned()
            .action
    )
}


pub fn check_authorities(
    authorities: &Vec<KeyBytes>,
    authorizations: &Vec<Authorization>,
    signed_content: &Vec<u8>,
) -> ExternResult<u8> {
    let mut sig_count : u8 = 0;

    debug!(
        "Checking {} authorization signature(s) out of {} authoritie(s)",
        authorizations.len(),
        authorities.len(),
    );
    for (auth_index, signature) in authorizations {
        let pubkey_bytes = authorities.get( *auth_index as usize )
            .ok_or(guest_error!(format!(
                "Auth index ({}) doesn't exist in authorities list: {:#?}",
                auth_index, authorities,
            )))?;

        let authority = AgentPubKey::from_raw_32( pubkey_bytes.to_owned().to_vec() );
        debug!(
            "Checking signature against authority: {}",
            authority,
        );
        if verify_signature_raw(
            authority,
            signature.to_owned(),
            signed_content.to_owned(),
        )? == false {
            Err(guest_error!("Authorization has invalid signature".to_string()))?
        }

        sig_count += 1;
    }

    Ok( sig_count )
}


pub fn keybytes_from_agentpubkey(
    agent: &AgentPubKey,
) -> ExternResult<KeyBytes> {
    agent.get_raw_32().try_into()
        .map_err( |e| wasm_error!(WasmErrorInner::Guest(format!(
            "Failed AgentPubKey to [u8;32] conversion: {:?}", e
        ))) )
}
