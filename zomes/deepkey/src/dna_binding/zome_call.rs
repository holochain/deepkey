use hdk::prelude::*;
use crate::dna_binding::entry::DnaKeyBinding;

#[hdk_extern]
fn create_dna_key_binding(new_dna_key_binding: DnaKeyBinding) -> ExternResult<HeaderHash> {
    create_entry(new_dna_key_binding)
}

#[hdk_extern]
fn install_an_app(_app_info: AppInfo) -> ExternResult<()> {
    // @todo
    Ok(())
}