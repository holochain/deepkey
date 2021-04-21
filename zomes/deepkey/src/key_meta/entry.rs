use hdk::prelude::*;

pub const DERIVATION_PATH_LEN: usize = 32;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
enum KeyType {
    AppUI,
    AppSig,
    AppEncryption,
    TLS,
}

#[derive(Debug)]
struct DerivationPath([u8; DERIVATION_PATH_LEN]);

fixed_array_serialization!(DerivationPath, DERIVATION_PATH_LEN);

#[hdk_entry(id = "key_meta", visibility = "private")]
pub struct KeyMeta {
    // references a KeyRegistration
    new_key: HeaderHash,
    derivation_path: DerivationPath,
    derivation_index: u32,
    key_type: KeyType,
}