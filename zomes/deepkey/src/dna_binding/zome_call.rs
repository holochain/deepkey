use hdk::prelude::*;

#[hdk_extern]
fn create_dna_key_binding(new_dna_key_binding: DnaKeyBinding) -> ExternResult<HeaderHash> {
    create_entry(new_dna_key_binding)
}

#[hdk_extern]
fn install_an_app(app_info: AppInfo) -> ExternResult<()> {
    // @todo
    Ok(())
}