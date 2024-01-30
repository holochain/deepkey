use deepkey::*;
use serde_bytes::ByteArray;

use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
    },
};


#[hdk_entry_helper]
pub enum KeyState {
    NotFound,
    Invalidated(SignedActionHashed),
    Valid(SignedActionHashed),
}


#[hdk_extern]
pub fn key_state((key_bytes, timestamp): (ByteArray<32>, Timestamp)) -> ExternResult<KeyState> {
    let key_anchor = KeyAnchor::new( key_bytes.into_array() );
    let key_anchor_hash = hash_entry( &key_anchor )?;
    let key_anchor_details = get_details( key_anchor_hash.clone(), GetOptions::default() )?;

    if let Some(details) = key_anchor_details {
        match details {
            Details::Entry(entry_details) => {
                if let Some(delete_record) = entry_details.deletes.first() {
                    if delete_record.action().timestamp() < timestamp {
                        return Ok(KeyState::Invalidated( delete_record.to_owned() ));
                    }
                }

                if let Some(record) = entry_details.actions.first() {
                    return Ok(KeyState::Valid( record.to_owned() ));
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
