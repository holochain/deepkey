use hdk::prelude::*;
use crate::meta::entry::KeyMeta;

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