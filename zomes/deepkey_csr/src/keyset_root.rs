use crate::utils;
use crate::hdi_extensions::{
    guest_error,
    ScopedTypeConnector,
};
use crate::hdk_extensions::{
    agent_id,
    must_get,
};

use deepkey::*;
use hdk::prelude::*;


#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<ActionHash> {
    // let membrane_proof = utils::my_membrane_proof()?;
    // debug!("Membrane proof: {:?}", membrane_proof );

    let fda: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let fda_bytes = fda.clone().into_inner();

    let esigs = sign_ephemeral_raw(vec![ fda_bytes ])?;
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

    init_change_rule( 1, vec![
        fda.get_raw_32().try_into()
            .map_err(|e| guest_error!(format!(
                "FDA.get_raw_32() did not have 32 elements; this should be unreachable -> {}", e
            )))?,
    ])?;

    Ok( create_hash )
}


pub fn init_change_rule(
    sigs_required: u8,
    revocation_keys: Vec<[u8; 32]>
) -> ExternResult<ActionHash> {
    let keyset_root_hash = utils::query_keyset_root_addr()?;
    let new_authority_spec = AuthoritySpec::new(
        sigs_required,
        revocation_keys,
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

    Ok( crate::change_rule::create_change_rule( change_rule )? )
}


#[hdk_extern]
pub fn get_keyset_root(ksr_addr: ActionHash) -> ExternResult<KeysetRoot> {
    must_get( &ksr_addr )?.try_into()
}


#[hdk_extern]
pub fn get_ksr_dia_links(ksr_addr: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            ksr_addr,
            LinkTypes::KeysetRootToDeviceInviteAcceptances,
        )?.build()
    )
}


#[hdk_extern]
pub fn get_device_key_anchor_links(author: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            author,
            LinkTypes::DeviceToKeyAnchor,
        )?.build()
    )
}


#[hdk_extern]
pub fn get_ksr_dia_members(ksr_addr: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    Ok(
        get_ksr_dia_links( ksr_addr.clone() )?
            .into_iter()
            .filter_map( |link| must_get( &link.target.into_any_dht_hash()? ).ok() )
            .map( |record| record.action().author().to_owned() )
            .collect()
    )
}


#[hdk_extern]
pub fn get_device_keys(author: AgentPubKey) -> ExternResult<Vec<KeyAnchor>> {
    Ok(
        get_device_key_anchor_links( author.clone() )?
            .into_iter()
            .filter_map( |link| must_get( &link.target.into_any_dht_hash()? ).ok() )
            .filter_map( |record| KeyAnchor::try_from( record ).ok() )
            .collect()
    )
}


#[hdk_extern]
pub fn get_ksr_members(ksr_addr: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    let ksr_record = must_get( &ksr_addr )?;
    let ksr_author = ksr_record.action().author().to_owned();
    let mut members = get_ksr_dia_members( ksr_addr.clone() )?;

    members.insert( 0, ksr_author );

    Ok( members )
}


// Get all of the members of the keyset: the first deepkey agent, and all the deepkey agents
#[hdk_extern]
pub fn query_keyset_members(keyset_root_hash: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    // Get the PubKey of the Deepkey Agent who wrote the KeysetRoot
    let keyset_root_record = must_get( &keyset_root_hash )?;
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
    let key_registrations = query_keyset_key_registration_records(keyset_root_hash)?
        .into_iter()
        .map(|key_reg_record| {
            let author = key_reg_record.action().author().clone();

            Ok(( author, KeyRegistration::try_from( key_reg_record )? ))
        })
        .collect::<ExternResult<Vec<(AgentPubKey, KeyRegistration)>>>()?;

    Ok(key_registrations)
}

// Get all of the keys registered on the keyset, across all the deepkey agents
#[hdk_extern]
pub fn query_keyset_keys(keyset_root_hash: ActionHash) -> ExternResult<Vec<KeyRegistration>> {
    let key_registrations = query_keyset_key_registration_records(keyset_root_hash)?
        .into_iter()
        .map( |key_reg_record| KeyRegistration::try_from( key_reg_record ) )
        .collect::<ExternResult<Vec<KeyRegistration>>>()?;

    Ok(key_registrations)
}

// Get all of the keys registered on the keyset, across all the deepkey agents
pub fn query_keyset_key_registration_records(
    keyset_root_hash: ActionHash
) -> ExternResult<Vec<Record>> {
    let key_registration_records = get_links(
        GetLinksInputBuilder::try_new(
            keyset_root_hash,
            LinkTypes::KeysetRootToKeyAnchors,
        )?.build()
    )?
        .into_iter()
        .filter_map( |link| link.target.into_entry_hash() )
        .map( |key_anchor_hash| must_get( &key_anchor_hash ) )
        .collect::<ExternResult<Vec<Record>>>()?
        .into_iter()
        .map( |record| record.action().prev_action().cloned() ) // Because the previous action should always be the registration record
        .filter_map(|x| x) // Drop anything without a prev action (should be unreachable)
        .map( |key_reg_actionhash| must_get( &key_reg_actionhash ) )
        .collect::<ExternResult<Vec<Record>>>()?;

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
        .filter_map( |link| link.target.into_entry_hash() )
        .map( |key_anchor_hash| must_get( &key_anchor_hash ) )
        .collect::<ExternResult<Vec<Record>>>()?
        .into_iter()
        .map( |key_anchor_record| KeyAnchor::try_from( key_anchor_record ) )
        .collect::<ExternResult<Vec<KeyAnchor>>>()?;

    Ok(key_anchors)
}
