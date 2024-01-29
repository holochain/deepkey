use hdi::prelude::{
    *,
    holo_hash::DnaHash
};


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct AppBinding {
    // TODO: if an app binding will not change for the series of registration updates, it doesn't
    // make sense to point to 1 key meta when there could be many in the series.
    pub key_meta_addr: ActionHash,
    pub dna_hashes: Vec<DnaHash>, //The hash of the DNA the key is bound to
    pub app_name: String,
}
