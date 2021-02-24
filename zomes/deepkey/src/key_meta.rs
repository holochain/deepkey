use hdk::prelude::*;
use crate::key_registration;

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
    new_key: key_registration::KeyRegistration,
    derivation_path: DerivationPath,
    key_type: KeyType,
}