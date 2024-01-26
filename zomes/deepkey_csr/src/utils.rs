use crate::hdi_extensions;
use std::sync::Arc;
use rmp_serde;

use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;

pub use crate::source_of_authority::*;
pub use deepkey::{
    utils::*,
    MembraneProof,
};


pub fn query_entry_type_latest<T,E>(unit: T) -> ExternResult<Option<Record>>
where
    EntryType: TryFrom<T, Error = E>,
    WasmError: From<E>,
{
    Ok(
        query(
            ChainQueryFilter::new()
                .include_entries(true)
                .entry_type( EntryType::try_from(unit)? )
        )?.pop()
    )
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


pub fn my_membrane_proof() -> ExternResult<Option<MembraneProof>> {
    Ok(match my_agent_validation_pkg()?.membrane_proof {
        Some(arc) => {
            match Arc::<SerializedBytes>::into_inner( arc ) {
                Some(serialized) => {
                    let proof : MembraneProof = rmp_serde::decode::from_slice( serialized.bytes().as_slice() )
                        .map_err(|e| guest_error!(format!("Failed membrane deserialization: {}", e )))?;
                    Some(proof)
                },
                _ => None,
            }
        },
        _ => None,
    })
}
