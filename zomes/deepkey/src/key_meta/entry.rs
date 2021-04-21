use hdk::prelude::*;

pub const DERIVATION_PATH_LEN: usize = 32;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub enum KeyType {
    AppUI,
    AppSig,
    AppEncryption,
    TLS,
}

#[derive(Debug)]
pub struct DerivationPath([u8; DERIVATION_PATH_LEN]);

fixed_array_serialization!(DerivationPath, DERIVATION_PATH_LEN);

#[hdk_entry(id = "key_meta", visibility = "private")]
pub struct KeyMeta {
    // references a KeyRegistration
    new_key: HeaderHash,
    derivation_path: DerivationPath,
    derivation_index: u32,
    key_type: KeyType,
}

impl TryFrom<&Element> for KeyMeta {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a KeyMeta.
            Header::Create(_) => {
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

impl KeyMeta {
    pub fn as_new_key_ref(&self) -> &HeaderHash {
        &self.new_key
    }

    pub fn as_derivation_path_ref(&self) -> &DerivationPath {
        &self.derivation_path
    }

    pub fn as_derivation_index_ref(&self) -> &u32 {
        &self.derivation_index
    }

    pub fn as_key_type(&self) -> &KeyType {
        &self.key_type
    }
}