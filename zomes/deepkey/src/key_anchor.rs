use hdi::prelude::*;
use hdi_extensions::{
    guest_error,
};


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyAnchor {
    pub bytes: [u8; 32],
}

impl KeyAnchor {
    pub fn from_agent_key(agent_key: AgentPubKey) -> ExternResult<Self> {
        let bytes: [u8; 32] = agent_key.get_raw_32()
            .try_into()
            .map_err( |e| guest_error!(format!(
                "Failed AgentPubKey to [u8;32] conversion: {:?}", e
            )) )?;

        Ok( Self { bytes } )
    }
}
