pub mod zome_call;

use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
use deepkey_integrity::device_authorization::device_invite::error::Error;
use deepkey_integrity::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use deepkey_integrity::entry::UnitEntryTypes;

pub fn local_keyset_parent() -> ExternResult<(ActionHash, ActionHash)> {

    // Lets find the EntryTypeIndex and ZomeId of the target Entry type:
    let dia_scoped_index: ScopedEntryDefIndex = UnitEntryTypes::DeviceInviteAcceptance.try_into().unwrap();
    let device_invite_acceptance_type = EntryType::App(AppEntryType::new(
        dia_scoped_index.zome_type, dia_scoped_index.zome_id, EntryVisibility::Public,
    ));

    let device_invite_acceptance_query = ChainQueryFilter::new().entry_type(device_invite_acceptance_type);
    match query(device_invite_acceptance_query)?.iter().next() {
        Some(device_invite_acceptance_element) => {
            let device_invite_acceptance = DeviceInviteAcceptance::try_from(device_invite_acceptance_element)?;
            Ok((device_invite_acceptance.keyset_root_authority, device_invite_acceptance_element.action_hashed().as_hash().to_owned()))
        },
        None => {
            let keyset_root_query = ChainQueryFilter::new().sequence_range(ChainQueryFilterRange::ActionSeqRange(KEYSET_ROOT_CHAIN_INDEX, KEYSET_ROOT_CHAIN_INDEX+1));
            match query(keyset_root_query)?.iter().next() {
                Some(keyset_root_element) => {
                    let action_hash = keyset_root_element.action_hashed().as_hash();
                    Ok((action_hash.clone(), action_hash.clone()))
                },
                None => return Err(Error::MissingKeyset.into()),
            }
        }
    }
}
