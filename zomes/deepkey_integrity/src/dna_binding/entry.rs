use hdk::prelude::*;
use hdk::prelude::holo_hash::DnaHash;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DNA_BINDING_INDEX: EntryDefIndex = EntryDefIndex(4);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AppName {
    bundle_name: String,
    cell_nick: String,
}

//#[hdk_entry(id = "dna_binding", visibility = "private")]
#[hdk_entry_helper]
pub struct DnaBinding {
    // A KeyMeta element
    key_meta: ActionHash,
    dna_hash: DnaHash,
    app_name: AppName,
}
/*
 * TODO: How do we limit to Create only?
 * 
impl TryFrom<&Record> for DnaBinding {
    type Error = crate::error::Error;
    fn try_from(element: &Record) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a DnaBinding.
            Action::Create(_) => {
                Ok(match element.entry() {
                    RecordEntry::Present(serialized) => match Self::try_from(serialized) {
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
 */
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
