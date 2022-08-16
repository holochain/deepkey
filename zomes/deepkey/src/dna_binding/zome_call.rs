use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::dna_binding::entry::DnaBinding;

#[hdk_extern]
fn new_dna_binding(new_dna_binding: DnaBinding) -> ExternResult<HeaderHash> {
    create_entry(new_dna_binding)
}

#[hdk_extern]
fn install_an_app(_app_info: AppInfo) -> ExternResult<()> {
    // @todo
    Ok(())
}
