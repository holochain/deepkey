use hdi::prelude::*;


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub enum KeyType {
    AppUI,
    AppSig,
    AppEncryption,
    TLS,
}


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyMeta {
    pub new_key: ActionHash, // Referencing a KeyRegistration by its ActionHash
    pub derivation_path: [u8; 32],
    pub derivation_index: u32,
    pub key_type: KeyType,
}
