use hdk::prelude::*;

pub const KEY_ANCHOR_BYTES: usize = 32;

#[derive(Clone, Copy, Debug, SerializedBytes)]
pub struct KeyAnchor([u8; KEY_ANCHOR_BYTES]);

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