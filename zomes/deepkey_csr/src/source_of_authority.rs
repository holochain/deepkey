use crate::utils;
use crate::hdi_extensions::{
    guest_error,
};
use crate::hdk_extensions::{
    must_get,
};

use deepkey::*;
use hdk::prelude::*;


// This function queries for the keyset authority for this conductor.
// It first checks if a device invite acceptance has been committed to the DHT
// If this is the case, we return the keyset root authority action hash from
// the device invite acceptance entry
// If this is not the case, we find and return the actual keyset root entry on this chain.
#[hdk_extern]
pub fn query_keyset_authority_action_hash(_: ()) -> ExternResult<ActionHash> {
    match utils::query_entry_type_latest( EntryTypesUnit::DeviceInviteAcceptance )? {
        Some(dia_record) => {
            let device_invite_acceptance : DeviceInviteAcceptance = dia_record.try_into()?;

            Ok(device_invite_acceptance.keyset_root_authority)
        },
        None => query_keyset_root_action_hash(()),
    }
}

// This function queries for the keyset root, and returns its action hash.
#[hdk_extern]
pub fn query_keyset_root_action_hash(_: ()) -> ExternResult<ActionHash> {
    match utils::query_entry_type_latest( EntryTypesUnit::KeysetRoot )? {
        Some(keyset_root) => Ok(keyset_root.action_address().to_owned()),
        None => Err(guest_error!("No KeysetFound on chain".to_string()))
    }
}


pub fn query_keyset_root_addr() -> ExternResult<ActionHash> {
    query_keyset_root_action_hash(())
}


pub fn query_keyset_root() -> ExternResult<KeysetRoot> {
    must_get( &query_keyset_root_addr()? )?.try_into()
}
