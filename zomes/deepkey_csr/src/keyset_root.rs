use serde_bytes;
use crate::utils;
use crate::hdi_extensions::{
    guest_error,
    ScopedTypeConnector,
};
use crate::hdk_extensions::{
    agent_id,
};
use crate::source_of_authority::{
    query_keyset_root_addr,
};

use deepkey::*;
use hdk::prelude::*;


#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<ActionHash> {
    let fda: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let fda_bytes = utils::serialize( &fda )?;

    let esigs = sign_ephemeral(vec![ fda_bytes ])?;
    let [signed_fda, ..] = esigs.signatures.as_slice() else {
        return Err(guest_error!("sign_ephemeral returned wrong number of signatures".to_string()))
    };

    let keyset_root = KeysetRoot::new(
        fda.clone(),
        esigs.key.get_raw_32().try_into()
            .map_err( |e| guest_error!(format!("Failed AgentPubKey to [u8;32] conversion: {:?}", e)) )?,
        signed_fda.to_owned()
    );
    let create_hash = create_entry( keyset_root.to_input() )?;

    Ok( create_hash )
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ChangeRuleInput {
    pub sigs_required: u8,
    pub revocation_keys: Vec<serde_bytes::ByteArray<32>>,
}

#[hdk_extern]
pub fn init_change_rule(input: ChangeRuleInput) -> ExternResult<ActionHash> {
    let keyset_root_hash = query_keyset_root_addr()?;
    let new_authority_spec = AuthoritySpec::new(
        input.sigs_required,
        input.revocation_keys.iter()
            .map(|key| key.into_array() )
            .collect(),
    );
    let auth_spec_bytes = utils::serialize( &new_authority_spec )?;
    let signed_bytes = sign( agent_id()?, auth_spec_bytes )?;

    let spec_change = AuthorizedSpecChange::new(
        new_authority_spec, vec![(0, signed_bytes)]
    );

    let change_rule = ChangeRule::new(
        keyset_root_hash.clone(),
        keyset_root_hash.clone(),
        spec_change,
    );
    let create_hash = create_entry( change_rule.to_input() )?;

    Ok( create_hash )
}


#[hdk_extern]
pub fn get_keyset_root(keyset_root_hash: ActionHash) -> ExternResult<Option<KeysetRoot>> {
    // get(keyset_root_hash, GetOptions::default())
    Ok(
        get(keyset_root_hash, GetOptions::default())?
            .and_then( |record| KeysetRoot::try_from( record ).ok() )
    )
}


// Get all of the members of the keyset: the first deepkey agent, and all the deepkey agents
#[hdk_extern]
pub fn query_keyset_members(keyset_root_hash: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    // Get the PubKey of the Deepkey Agent who wrote the KeysetRoot
    let keyset_root_record =
        get(keyset_root_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the Keyset Root"))
        ))?;
    let ksr_chain_pubkey = keyset_root_record.action().author().clone();

    let dia_hashes: Vec<ActionHash> = get_links(
        GetLinksInputBuilder::try_new(
            keyset_root_hash,
            LinkTypes::KeysetRootToDeviceInviteAcceptances,
        )?.build()
    )?
    .into_iter()
    .filter_map(|link| link.target.into_action_hash())
    .collect();
    let dia_records: Vec<Record> = dia_hashes
        .into_iter()
        .map(|dia_hash| get(dia_hash, GetOptions::default()))
        .collect::<ExternResult<Vec<Option<Record>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect();

    // Query all the Deepkey Agents that wrote the DeviceInviteAcceptances
    let mut dia_author_pubkeys = dia_records
        .into_iter()
        .map(|dia_record| dia_record.action().author().clone())
        .collect::<Vec<AgentPubKey>>();

    // Don't forget the First Deepkey Agent
    dia_author_pubkeys.push(ksr_chain_pubkey);

    Ok(dia_author_pubkeys)
}

#[hdk_extern]
pub fn query_keyset_keys_with_authors(
    keyset_root_hash: ActionHash,
) -> ExternResult<Vec<(AgentPubKey, KeyRegistration)>> {
    let key_registrations = _query_keyset_key_records(keyset_root_hash)?
        .into_iter()
        .map(|key_reg_record| {
            let author = key_reg_record.action().author().clone();
            key_reg_record
                .entry
                .to_app_option::<KeyRegistration>()
                .map_err(|e| {
                    wasm_error!(WasmErrorInner::Guest(format!(
                        "Could not deserialize KeyRegistration entry: {}",
                        e
                    )))
                })
                .map(|opt| opt.map(|key_reg| (author, key_reg)))
        })
        .collect::<ExternResult<Vec<Option<(AgentPubKey, KeyRegistration)>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<(AgentPubKey, KeyRegistration)>>();
    Ok(key_registrations)
}

// Get all of the keys registered on the keyset, across all the deepkey agents
#[hdk_extern]
pub fn query_keyset_keys(keyset_root_hash: ActionHash) -> ExternResult<Vec<KeyRegistration>> {
    let key_registrations = _query_keyset_key_records(keyset_root_hash)?
        .into_iter()
        .map(|key_reg_record| {
            key_reg_record
                .entry
                .to_app_option::<KeyRegistration>()
                .map_err(|e| {
                    wasm_error!(WasmErrorInner::Guest(format!(
                        "Could not deserialize KeyRegistration entry: {}",
                        e
                    )))
                })
        })
        .collect::<ExternResult<Vec<Option<KeyRegistration>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<KeyRegistration>>();
    Ok(key_registrations)
}

// Get all of the keys registered on the keyset, across all the deepkey agents
pub fn _query_keyset_key_records(keyset_root_hash: ActionHash) -> ExternResult<Vec<Record>> {
    let key_registration_records =
        get_links(
            GetLinksInputBuilder::try_new(
                keyset_root_hash,
                LinkTypes::KeysetRootToKeyAnchors,
            )?.build()
        )?
            .into_iter()
            .filter_map(|link| link.target.into_entry_hash())
            .map(|key_anchor_hash: EntryHash| get(key_anchor_hash, GetOptions::default()))
            .collect::<ExternResult<Vec<Option<Record>>>>()?
            .into_iter()
            .filter_map(|x| x) // Drop any dead links
            .map(|record| record.action().prev_action().cloned())
            .filter_map(|x| x) // Drop anything without a prev action
            .map(|key_reg_actionhash| get(key_reg_actionhash, GetOptions::default()))
            .collect::<ExternResult<Vec<Option<Record>>>>()?
            .into_iter()
            .filter_map(|x| x)
            .collect::<Vec<Record>>();

    Ok(key_registration_records)
}

#[hdk_extern]
pub fn query_keyset_key_anchors(keyset_root_hash: ActionHash) -> ExternResult<Vec<KeyAnchor>> {
    let key_anchors = get_links(
        GetLinksInputBuilder::try_new(
            keyset_root_hash,
            LinkTypes::KeysetRootToKeyAnchors,
        )?.build()
    )?
        .into_iter()
        .filter_map(|link| link.target.into_entry_hash())
        .map(|key_anchor_hash: EntryHash| get(key_anchor_hash, GetOptions::default()))
        .collect::<ExternResult<Vec<Option<Record>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .map(|key_anchor_record| {
            key_anchor_record
                .entry
                .to_app_option::<KeyAnchor>()
                .map_err(|e| {
                    wasm_error!(WasmErrorInner::Guest(format!(
                        "Could not deserialize KeyAnchor entry: {}",
                        e
                    )))
                })
        })
        .collect::<ExternResult<Vec<Option<KeyAnchor>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect::<Vec<KeyAnchor>>();

    Ok(key_anchors)
}
