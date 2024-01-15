use rmp_serde;
use crate::hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;


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


pub fn serialize<T>(target: &T) -> ExternResult<Vec<u8>>
where
    T: Serialize + ?Sized,
{
    rmp_serde::encode::to_vec( target )
        .map_err( |err| guest_error!(format!(
            "Failed to serialize target: {:?}", err
        )) )
}
