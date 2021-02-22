hdk::prelude::*;

use crate::key_meta;

struct AppName {
    bundle_name: String,
    cell_nick: String,
}

#[hdk_entry(id = "dna_key_binding", visibility = "private")]
struct DnaKeyBinding {
    key: key_meta::KeyMeta,
    dna_hash: DnaHash,
    app_name: AppName,
}

#[hdk_extern]
fn create_dna_key_binding(new_dna_key_binding: DnaKeyBinding) -> ExternResult<HeaderHash> {
    create_entry(new_dna_key_binding)
}