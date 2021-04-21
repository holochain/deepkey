use hdk::prelude::*;
use crate::meta::entry::KeyMeta;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DNA_BINDING_INDEX: EntryDefIndex = EntryDefIndex(4);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AppName {
    bundle_name: String,
    cell_nick: String,
}

#[hdk_entry(id = "dna_binding", visibility = "private")]
pub struct DnaBinding {
    key: KeyMeta,
    dna_hash: hdk::prelude::holo_hash::DnaHash,
    app_name: AppName,
}

impl TryFrom<&Element> for DnaBinding {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DnaBinding.
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

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::DNA_BINDING_INDEX;
    use super::DnaBinding;

    #[test]
    fn dna_key_binding_index_test() {
        assert_eq!(
            DNA_BINDING_INDEX,
            entry_def_index!(DnaBinding).unwrap()
        )
    }
}