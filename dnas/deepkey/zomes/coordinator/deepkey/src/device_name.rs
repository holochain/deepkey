use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn name_device(device_name: String) -> ExternResult<()> {
    let agent_pubkey = agent_info()?.agent_latest_pubkey;

    create_link(
        agent_pubkey.clone(),
        agent_pubkey,
        LinkTypes::DeviceName,
        LinkTag::new(device_name),
    )?;

    Ok(())
}

#[hdk_extern]
pub fn get_device_name(agent_pubkey: AgentPubKey) -> ExternResult<Option<String>> {
    let links = get_links(agent_pubkey.clone(), LinkTypes::DeviceName, None)?;

    match links.first() {
        Some(link) => {
            let name = String::from_utf8(link.tag.clone().into_inner()).map_err(|_| {
                wasm_error!(WasmErrorInner::Guest(
                    "Can't parse the name from this link.".into()
                ))
            })?;
            Ok(Some(name))
        }
        None => Ok(None),
    }
}
