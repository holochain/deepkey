use crate::{
    utils,
    deepkey_sdk,
};
use deepkey::*;
use deepkey_sdk::{
    KeyState,
    KeyBytes,
};
use serde_bytes::ByteArray;

use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
    must_get_record_details,
    hdi_extensions::{
        guest_error,
        trace_origin,
        trace_origin_root,
        ScopedTypeConnector,
    },
};


#[hdk_extern]
pub fn create_key_anchor(key_anchor: KeyAnchor) -> ExternResult<ActionHash> {
    let create_addr = create_entry( key_anchor.to_input() )?;

    create_link(
        agent_id()?,
        create_addr.clone(),
        LinkTypes::DeviceToKeyAnchor,
        "create".as_bytes().to_vec(),
    )?;

    Ok( create_addr )
}


#[hdk_extern]
pub fn key_state((key_bytes, timestamp): (ByteArray<32>, Timestamp)) -> ExternResult<KeyState> {
    let key_anchor = KeyAnchor::new( key_bytes.into_array() );
    let key_anchor_hash = hash_entry( &key_anchor )?;
    let key_anchor_details = get_details( key_anchor_hash.clone(), GetOptions::default() )?;

    if let Some(details) = key_anchor_details {
        match details {
            Details::Entry(entry_details) => {
                debug!(
                    "Details for KeyAnchor entry '{}': {} create(s), {} update(s), {} delete(s)",
                    key_anchor_hash,
                    entry_details.actions.len(),
                    entry_details.updates.len(),
                    entry_details.deletes.len(),
                );
                if let Some(delete_record) = entry_details.deletes.first() {
                    if delete_record.action().timestamp() < timestamp {
                        return Ok(KeyState::Invalid( Some(delete_record.to_owned()) ));
                    }
                    else {
                        debug!(
                            "Deletion occurred after the given timestamp: [deleted] {} > {}",
                            delete_record.action().timestamp(), timestamp
                        );
                    }
                }

                if let Some(update_record) = entry_details.updates.first() {
                    if update_record.action().timestamp() < timestamp {
                        return Ok(KeyState::Invalid( Some(update_record.to_owned()) ));
                    }
                    else {
                        debug!(
                            "Update occurred after the given timestamp: [updated] {} > {}",
                            update_record.action().timestamp(), timestamp
                        );
                    }
                }

                if let Some(record) = entry_details.actions.first() {
                    return Ok(
                        match record.action().timestamp() > timestamp {
                            true => {
                                debug!(
                                    "Create occurred after the given timestamp: [created] {} > {}",
                                    record.action().timestamp(), timestamp
                                );
                                KeyState::Invalid( None )
                            },
                            false => KeyState::Valid( record.to_owned() ),
                        }
                    );
                }

                Err(guest_error!("KeyAnchor anchor details has no actions".into()))?
            },
            Details::Record(_) => Err(guest_error!("Problem with get_details result".into()))?,
        }
    }
    else {
        Ok( KeyState::NotFound )
    }
}


#[hdk_extern]
pub fn get_key_anchor_for_registration(addr: ActionHash) -> ExternResult<(ActionHash, KeyAnchor)> {
    // Get registration
    let key_registration = crate::key_registration::get_key_registration( addr )?;

    // Derive key anchor entry hash
    let key_anchor_hash = key_registration.key_anchor_hash()?;

    // Get record for key anchor
    let record = must_get( &key_anchor_hash )?;

    Ok((
        record.action_address().to_owned(),
        record.try_into()?
    ))
}


#[hdk_extern]
pub fn query_action_addr_for_key_anchor(
    key_bytes: ByteArray<32>
) -> ExternResult<ActionHash> {
    let key = key_bytes.into_array();
    let key_anchor = KeyAnchor::new( key.clone() );
    let key_anchor_hash = hash_entry( &key_anchor )?;

    let key_anchor_addr = utils::query_entry_type( EntryTypesUnit::KeyAnchor )?
        .into_iter()
        .filter_map( |record| Some(
            (
                record.action_address().to_owned(),
                record.action().entry_hash()?.to_owned(),
            )
        ))
        .find( |(_, hash)| hash == &key_anchor_hash )
        .ok_or(guest_error!(format!("No KeyMeta for anchor hash: {}", key_anchor_hash )))?.0;

    Ok( key_anchor_addr )
}


#[hdk_extern]
pub fn get_first_key_anchor_for_key(key_bytes: ByteArray<32>) -> ExternResult<(ActionHash, KeyAnchor)> {
    let ka_action_addr = query_action_addr_for_key_anchor( key_bytes )?;
    let first_ka_action_addr = trace_origin_root( &ka_action_addr )?.0;

    Ok((
        first_ka_action_addr.clone(),
        must_get( &first_ka_action_addr )?.try_into()?,
    ))
}


#[hdk_extern]
pub fn query_key_lineage(
    key_bytes: ByteArray<32>,
) -> ExternResult<Vec<KeyBytes>> {
    let ka_action_addr = query_action_addr_for_key_anchor( key_bytes )?;
    let key_meta = crate::key_meta::query_key_meta_for_key_addr( ka_action_addr )?;
    let key_metas = crate::key_meta::query_key_metas_for_app_binding( key_meta.app_binding_addr )?;

    let mut lineage = vec![];

    debug!("Lineage has {} keys", key_metas.len() );
    for key_meta in key_metas {
        let key_anchor : KeyAnchor = must_get_record_details( &key_meta.key_anchor_addr )?
            .record
            .try_into()?;

        lineage.push( key_anchor.bytes );
    }

    Ok( lineage )
}


#[hdk_extern]
pub fn get_key_lineage(
    key_bytes: ByteArray<32>,
) -> ExternResult<Vec<KeyBytes>> {
    let key_anchor = KeyAnchor::new( key_bytes.into_array() );
    let key_anchor_hash = hash_entry( &key_anchor )?;
    let key_anchor_details = get_details( key_anchor_hash.clone(), GetOptions::default() )?;

    let (creation_addr, mut next_addr) = match key_anchor_details {
        None => Err(guest_error!(format!("Record not found")))?,
        Some(Details::Record(_)) => Err(guest_error!(format!("Should be unreachable")))?,
        Some(Details::Entry( entry_details )) => {
            (
                entry_details.actions.first()
                    .ok_or(guest_error!(format!("Should be unreachable")))?
                    .action_address()
                    .to_owned(),
                entry_details.updates.first()
                    .map( |action| action.action_address().to_owned() ),
            )
        },
    };

    let mut lineage = vec![];

    // Trace backwards
    let history = trace_origin( &creation_addr )?;

    debug!(
        "Found history ({}): {:?}",
        history.len(),
        history.clone().iter().map( |(addr, _)| addr ).collect::<Vec<&ActionHash>>(),
    );
    for (addr, _action) in history {
        let key_anchor : KeyAnchor = must_get_record_details( &addr )?
            .record
            .try_into()?;

        lineage.push( key_anchor.bytes );
    }

    lineage.reverse();

    // Follow evolutions forwards
    while let Some(addr) = next_addr {
        let details = must_get_record_details( &addr )?;
        let key_anchor : KeyAnchor = details.record.try_into()?;

        debug!("Found update ({}): {:?}", addr, key_anchor.bytes );
        lineage.push( key_anchor.bytes );

        next_addr = details.updates.first()
            .map( |action| action.action_address().to_owned() );
    }

    Ok( lineage )
}


#[hdk_extern]
pub fn query_same_lineage(
    (key1_bytes, key2_bytes): (ByteArray<32>, ByteArray<32>),
) -> ExternResult<bool> {
    let keys = query_key_lineage( key1_bytes )?;

    Ok( keys.contains( &key2_bytes.into_array() ) )
}


#[hdk_extern]
pub fn same_lineage(
    (key1_bytes, key2_bytes): (ByteArray<32>, ByteArray<32>),
) -> ExternResult<bool> {
    let keys = get_key_lineage( key1_bytes )?;

    Ok( keys.contains( &key2_bytes.into_array() ) )
}
