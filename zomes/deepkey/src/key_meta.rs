use hdk::prelude::*;
use crate::key_registration;

enum KeyType {
    AppUI,
    AppSig,
    AppEncryption,
    TLS,
}

struct DerivationPath(#[serde(with = "serde_bytes")] [u8; 32]);

#[hdk_entry(id = "key_meta", visibility = "private")]
struct KeyMeta {
    new_key: key_registration::KeyRegistration,
    derivation_path: derivation_path::DerivationPath,
    key_type: KeyType,
}