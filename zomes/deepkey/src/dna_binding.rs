use hdi::prelude::{
    *,
    holo_hash::hash_type::Dna
};


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct DnaBinding {
    pub key_meta: ActionHash, // Referencing a KeyMeta by its ActionHash
    pub dna_hash: HoloHash<Dna>, //The hash of the DNA the key is bound to
    pub app_name: String,
}
