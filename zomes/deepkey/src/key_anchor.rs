use hdi::prelude::*;


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyAnchor {
    pub bytes: [u8; 32],
}

impl KeyAnchor {
    pub fn from_agent_key(agent_key: AgentPubKey) -> Self {
        let slice = agent_key.get_raw_32();
        let bytes: [u8; 32] = match slice.try_into() {
            Ok(array) => array,
            Err(_) => panic!("Failed to convert AgentPubKey to [u8; 32]"),
        };
        Self { bytes }
    }
}
