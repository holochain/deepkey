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
