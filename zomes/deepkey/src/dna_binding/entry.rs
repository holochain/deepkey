use hdk::prelude::*;
use crate::meta::entry::KeyMeta;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const DNA_KEY_BINDING_INDEX: EntryDefIndex = EntryDefIndex(4);

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AppName {
    bundle_name: String,
    cell_nick: String,
}

#[hdk_entry(id = "dna_key_binding", visibility = "private")]
pub struct DnaKeyBinding {
    key: KeyMeta,
    dna_hash: hdk::prelude::holo_hash::DnaHash,
    app_name: AppName,
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::DNA_KEY_BINDING_INDEX;
    use super::DnaKeyBinding;

    #[test]
    fn dna_key_binding_index_test() {
        assert_eq!(
            DNA_KEY_BINDING_INDEX,
            entry_def_index!(DnaKeyBinding).unwrap()
        )
    }
}