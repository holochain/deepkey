use deepkey_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_authority_spec(authority_spec: AuthoritySpec) -> ExternResult<Record> {
    let authority_spec_hash = create_entry(&EntryTypes::AuthoritySpec(authority_spec.clone()))?;
    for base in authority_spec.signers.clone() {
        create_link(
            base,
            authority_spec_hash.clone(),
            LinkTypes::SignerToAuthoritySpecs,
            (),
        )?;
    }
    let record = get(authority_spec_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from(
            "Could not find the newly created AuthoritySpec"
        ))
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn get_authority_spec(authority_spec_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(authority_spec_hash, GetOptions::default())
}
#[hdk_extern]
pub fn get_authority_specs_for_signer(signer: AgentPubKey) -> ExternResult<Vec<Record>> {
    let links = get_links(signer, LinkTypes::SignerToAuthoritySpecs, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| GetInput::new(ActionHash::from(link.target).into(), GetOptions::default()))
        .collect();
    let records: Vec<Record> = HDK
        .with(|hdk| hdk.borrow().get(get_input))?
        .into_iter()
        .filter_map(|r| r)
        .collect();
    Ok(records)
}
