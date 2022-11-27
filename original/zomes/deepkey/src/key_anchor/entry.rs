use hdk::prelude::*;
use crate::key_registration::entry::KeyGeneration;

pub const KEY_ANCHOR_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, SerializedBytes)]
pub struct KeyAnchor([u8; KEY_ANCHOR_BYTES]);

impl From<&KeyGeneration> for KeyAnchor {
    fn from(key_registration: &KeyGeneration) -> Self {
        let mut bytes = [0; 32];
        bytes.copy_from_slice(key_registration.as_new_key_ref().get_raw_32());
        Self(bytes)
    }
}

entry_def!(KeyAnchor EntryDef {
    id: "key_anchor".into(),
    crdt_type: CrdtType::default(),
    required_validations: RequiredValidations::default(),
    required_validation_type: RequiredValidationType::default(),
    visibility: EntryVisibility::default(),
});

impl AsRef<[u8]> for KeyAnchor {
    fn as_ref(&self) -> &[u8] {
        self.0.as_ref()
    }
}

fixed_array_serialization!(KeyAnchor, KEY_ANCHOR_BYTES);

impl TryFrom<&Element> for KeyAnchor {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // All CRUD are allowed for a KeyAnchor.
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