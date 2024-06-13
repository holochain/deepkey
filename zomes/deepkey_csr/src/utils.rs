use crate::hdi_extensions;

use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    must_get_record_details,
};

pub use crate::source_of_authority::*;
pub use deepkey::{
    utils::*,
};


pub fn query_entry_type<T,E>(unit: T) -> ExternResult<Vec<Record>>
where
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    Ok(
        query(
            ChainQueryFilter::new()
                .include_entries(true)
                .entry_type( EntryType::try_from(unit)? )
        )?
    )
}


pub fn query_entry_type_first<T,E>(unit: T) -> ExternResult<Option<Record>>
where
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    Ok( query_entry_type( unit )?.pop() )
}


pub fn query_entry_type_latest<T,E>(unit: T) -> ExternResult<Option<Record>>
where
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    Ok( query_entry_type( unit )?.pop() )
}


pub fn get_chain_index(index: u32) -> ExternResult<Option<Record>> {
    Ok(
        query(
            ChainQueryFilter::new()
                .sequence_range( ChainQueryFilterRange::ActionSeqRange(index, index) )
        )?.pop()
    )
}


pub fn my_agent_validation_pkg() -> ExternResult<AgentValidationPkg> {
    let action = get_chain_index( 1 )?
        .ok_or(guest_error!("Chain is missing index 1".to_string()))?
        .signed_action.hashed.content;

    if let Action::AgentValidationPkg(avp) = action {
        Ok( avp )
    } else {
        Err(guest_error!("Chain index 1 is not an AgentValidationPkg action".to_string()))?
    }
}


pub fn get_next_update(addr: ActionHash) -> ExternResult<Option<ActionHash>> {
    let mut details = must_get_record_details( &addr )?;

    // Sort updates in ascending timestamp order so that the last item is the most recent update
    details.updates.sort_by( |a,b| {
        a.action().timestamp().to_owned().cmp(&b.action().timestamp())
    });

    Ok(
        match details.updates.last() {
            Some(update) => Some(update.action_address().to_owned()),
            None => None,
        }
    )
}


pub fn get_latest_record(addr: ActionHash) -> ExternResult<Record> {
    let mut latest_addr = addr;

    while let Some(update) = get_next_update( latest_addr.clone() )? {
        latest_addr = update;
    }

    must_get( &latest_addr )
}
