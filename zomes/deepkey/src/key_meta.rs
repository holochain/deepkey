use hdi::prelude::*;


// #[derive(Debug, Clone, Serialize, Deserialize)]
// pub enum KeyType {
//     AppUI,
//     AppSig,
//     AppEncryption,
//     TLS,
// }


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationDetails {
    pub app_index: u32,
    pub agent_index: u32,
}


#[hdk_entry_helper]
#[derive(Clone)]
pub struct KeyMeta {
    // TODO: make sure we can ensure there is only 1 key anchor creation action
    // pub key_anchor_hash: EntryHash,
    pub key_anchor_addr: ActionHash,

    pub derivation_details: DerivationDetails,
    // pub derivation_bytes: [u8; 32],
    // pub key_type: KeyType,
}
