// use crate::hdi_extensions::{
//     guest_error,
// };
use crate::hdk_extensions::{
    must_get,
};

use deepkey::*;
use hdk::prelude::*;


#[hdk_extern]
pub fn get_device_key_links(author: AgentPubKey) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            author,
            LinkTypes::DeviceToKeyAnchor,
        )?.build()
    )
}


#[hdk_extern]
pub fn get_device_keys(agent: AgentPubKey) -> ExternResult<Vec<(EntryHash, KeyAnchor)>> {
    Ok(
        get_device_key_links( agent )?
            .into_iter()
            .filter_map( |link| must_get( &link.target.into_any_dht_hash()? ).ok() )
            .filter_map( |record| Some((
                record.action().entry_hash()?.to_owned(),
                KeyAnchor::try_from( record ).ok()?,
            )))
            .collect()
    )
}
