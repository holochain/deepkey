use hdk::prelude::*;
use crate::key::entry::Key;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyAuthorization {
    key: Key,
    signature: Signature,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyRevocation {
    key: Key,
    signature: Signature,
}

#[hdk_entry(id = "key_registration")]
pub enum KeyRegistration {
    Create(KeyAuthorization),
    Update(KeyRevocation, KeyAuthorization),
    Delete(KeyRevocation)
}