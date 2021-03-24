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

impl TryFrom<&Element> for KeyRegistration {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // All CRUD are allowed for a Generator.
            Header::Create(_) | Header::Update(_) | Header::Delete(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}