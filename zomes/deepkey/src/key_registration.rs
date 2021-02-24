use hdk::prelude::*;
use crate::key;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyAuthorization {
    key: key::PubKey,
    signature: Signature,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyRevocation {
    key: key::PubKey,
    signature: Signature,
}

#[hdk_entry(id = "key_registration")]
pub enum KeyRegistration {
    Create(KeyAuthorization),
    Update(KeyRevocation, KeyAuthorization),
    Delete(KeyRevocation)
}

#[hdk_extern]
fn create_key_registration(key_registration: KeyRegistration) -> ExternResult<HeaderHash> {
    create_entry(key_registration)
}