use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::dna_binding::entry::DnaBinding;
use deepkey_integrity::entry::EntryTypes;

#[hdk_extern]
fn new_dna_binding(new_dna_binding: DnaBinding) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::DnaBinding(new_dna_binding))
}

#[hdk_extern]
fn install_an_app(_app_info: AppInfo) -> ExternResult<()> {
    // @todo
    Ok(())
}
