use hdk::prelude::*;
use crate::error::Error;
use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::entry::DeviceAuthorization;

// A deepkey agent is any agent that is either the First Deepkey Agent or anything referenced by a device handshake.
// Either the FDA or anything referenced by a device handshake.

pub enum DeepKeyAgent {
    Root(KeysetRoot),
    Device(DeviceAuthorization),
}

impl DeepKeyAgent {
    pub fn is_agent(&self, other: &AgentPubKey) -> bool {
        match self {
            DeepKeyAgent::Root(keyset_root) => keyset_root.as_first_deepkey_agent_ref() == other,
            DeepKeyAgent::Device(device_authorization) => {
                ( device_authorization.as_trusted_device_deepkey_agent_id_1_ref() == other ) || ( device_authorization.as_trusted_device_deepkey_agent_id_2_ref() == other )
            }
        }
    }
}

pub fn query() -> ExternResult<Vec<DeepKeyAgent>> {
    let root_query = ChainQueryFilter::new()
        .sequence_range(KEYSET_ROOT_CHAIN_INDEX..KEYSET_ROOT_CHAIN_INDEX+1)
        .entry_type(entry_type!(KeysetRoot)?)
        .include_entries(true);
    let mut deepkey_agents: Vec<DeepKeyAgent> = hdk::prelude::query(root_query)?
    .iter()
    .filter_map(|element| match element.entry().to_app_option::<KeysetRoot>() {
        Ok(Some(keyset_root)) => Some(DeepKeyAgent::Root(keyset_root)),
        Ok(None) => {
            error!(?element, "{}", Error::MissingKeysetRoot);
            None
        },
        Err(error) => {
            error!(?element, ?error, "{}", Error::MissingKeysetRoot);
            None
        },
    })
    .collect();

    if deepkey_agents.len() != 0 {
        let device_query = ChainQueryFilter::new()
            .entry_type(entry_type!(DeviceAuthorization)?)
            .include_entries(true);
        let mut devices: Vec<DeepKeyAgent> = hdk::prelude::query(device_query)?
        .iter()
        .filter_map(|element| match element.entry().to_app_option::<DeviceAuthorization>() {
            Ok(Some(device_authorization)) => Some(DeepKeyAgent::Device(device_authorization)),
            Ok(None) => {
                error!(?element, "{}", Error::MissingDevice);
                None
            },
            Err(error) => {
                error!(?element, ?error, "{}", Error::MissingDevice);
                None
            },
        })
        .collect();
        deepkey_agents.append(&mut devices);
    }
    else {
        error!("{}", Error::MissingKeysetRoot);
    }
    Ok(deepkey_agents)
}

