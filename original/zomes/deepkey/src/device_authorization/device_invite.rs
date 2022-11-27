pub mod entry;
pub mod validate;
pub mod error;
pub mod zome_call;

use hdk::prelude::*;
use error::Error;
use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;

pub fn local_keyset_parent() -> ExternResult<(HeaderHash, HeaderHash)> {
    let device_invite_acceptance_query = ChainQueryFilter::new().entry_type(entry_type!(DeviceInviteAcceptance)?);
    match query(device_invite_acceptance_query)?.iter().next() {
        Some(device_invite_acceptance_element) => {
            let device_invite_acceptance = DeviceInviteAcceptance::try_from(device_invite_acceptance_element)?;
            Ok((device_invite_acceptance.keyset_root_authority, device_invite_acceptance_element.header_hashed().as_hash().to_owned()))
        },
        None => {
            let keyset_root_query = ChainQueryFilter::new().sequence_range(KEYSET_ROOT_CHAIN_INDEX..KEYSET_ROOT_CHAIN_INDEX+1);
            match query(keyset_root_query)?.iter().next() {
                Some(keyset_root_element) => {
                    let header_hash = keyset_root_element.header_hashed().as_hash();
                    Ok((header_hash.clone(), header_hash.clone()))
                },
                None => return Err(Error::MissingKeyset.into()),
            }
        }
    }
}