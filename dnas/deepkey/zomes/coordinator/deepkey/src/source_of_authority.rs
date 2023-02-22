use deepkey_integrity::*;
use hdk::prelude::*;

// This function queries for the keyset authority, and returns its action hash.
// It first checks if a device invite acceptance has been committed to the DHT
// If this is the case, we return the keyset root authority action hash from
// the device invite acceptance entry
// If this is not the case, we find and return the actual keyset root entry on this chain.
// TODO: consider querying for KSR using range
// ChainQueryFilter::new().sequence_range(ChainQueryFilterRange::ActionSeqRange(KEYSET_ROOT_INDEX, KEYSET_ROOT_INDEX + 1))
#[hdk_extern]
pub fn query_keyset_authority_action_hash(_: ()) -> ExternResult<ActionHash> {
    if let Some(device_invite_acceptance) = query(
        ChainQueryFilter::new()
            .entry_type(UnitEntryTypes::DeviceInviteAcceptance.try_into().unwrap()),
    )?
    .into_iter()
    .next()
    {
        let device_invite_acceptance =
            DeviceInviteAcceptance::try_from(device_invite_acceptance.clone())?;
        Ok(device_invite_acceptance.keyset_root_authority)
    } else if let Some(keyset_root) =
        query(ChainQueryFilter::new().entry_type(UnitEntryTypes::KeysetRoot.try_into().unwrap()))?
            .into_iter()
            .next()
    {
        Ok(keyset_root.action_address().to_owned())
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "No KeysetFound on chain".into()
        )))
    }
}
// This function queries for the keyset root, and returns its action hash.
// TODO: consider querying for KSR using range
// ChainQueryFilter::new().sequence_range(ChainQueryFilterRange::ActionSeqRange(KEYSET_ROOT_INDEX, KEYSET_ROOT_INDEX + 1))
#[hdk_extern]
pub fn query_keyset_root_action_hash(_: ()) -> ExternResult<ActionHash> {
    if let Some(keyset_root) =
        query(ChainQueryFilter::new().entry_type(UnitEntryTypes::KeysetRoot.try_into().unwrap()))?
            .into_iter()
            .next()
    {
        Ok(keyset_root.action_address().to_owned())
    } else {
        Err(wasm_error!(WasmErrorInner::Guest(
            "No KeysetFound on chain".into()
        )))
    }
}
