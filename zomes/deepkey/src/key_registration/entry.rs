use hdk::prelude::*;
use crate::key::entry::Key;
use crate::change_rule::entry::Authorization;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyGeneration {
    new_key: Key,
    // Ensure the generator has the same author as the KeyRegistration.
    generator: HeaderHash,
    generator_signature: Signature,
}

impl KeyGeneration {
    pub fn as_new_key_ref(&self) -> &Key {
        &self.new_key
    }

    pub fn as_generator_ref(&self) -> &HeaderHash {
        &self.generator
    }

    pub fn as_generator_signature_ref(&self) -> &Signature {
        &self.generator_signature
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct KeyRevocation {
    prior_key_registration: HeaderHash,
    // To be validated according to the change rule of the generator of the prior key.
    revocation_authorization: Vec<Authorization>,
}

impl KeyRevocation {
    pub fn as_prior_key_registration_ref(&self) -> &HeaderHash {
        &self.prior_key_registration
    }

    pub fn as_revocation_authorization_ref(&self) -> &[Authorization] {
        &self.revocation_authorization
    }
}

#[hdk_entry(id = "key_registration")]
pub enum KeyRegistration {
    Create(KeyGeneration),
    Update(KeyRevocation, KeyGeneration),
    Delete(KeyRevocation)
}

impl TryFrom<&Element> for KeyRegistration {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // All CRUD are allowed for a KeyRegistration.
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