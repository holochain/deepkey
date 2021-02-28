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

/// Try not to do this because it is not efficient to deserialize if you know the type.
impl TryFrom<&Entry> for DeepKeyAgent {
    type Error = crate::error::Error;
    fn try_from(entry: &Entry) -> Result<Self, Self::Error> {
        match KeysetRoot::try_from(entry) {
            Ok(keyset_root) => Ok(DeepKeyAgent::Root(keyset_root)),
            Err(_) => match DeviceAuthorization::try_from(entry) {
                Ok(device_authorization) => Ok(DeepKeyAgent::Device(device_authorization)),
                Err(_) => Err(crate::error::Error::EntryNotDeepKeyAgent),
            }
        }
    }
}

impl DeepKeyAgent {
    pub fn is_agent(&self, other: &AgentPubKey) -> bool {
        match self {
            DeepKeyAgent::Root(keyset_root) => keyset_root.as_first_deepkey_agent_ref() == other,
            DeepKeyAgent::Device(device_authorization) => {
                ( device_authorization.as_root_acceptance_ref().0 == *other ) || ( device_authorization.as_device_acceptance_ref().0 == *other )
            }
        }
    }
}

// pub fn query() -> ExternResult<Option<DeepKeyAgent>> {
//     let q = ChainQueryFilter::new()
//         .sequence_range(KEYSET_ROOT_CHAIN_INDEX..KEYSET_ROOT_CHAIN_INDEX+1)
//         .include_entries(true);
//     let mut elements: Vec<Element> = hdk::prelude::query(q)?;

//     Ok(
//         elements.pop()
//             .and_then(|element| element.entry().into_option())
//             .and_then(|entry| DeepKeyAgent::try_from(&entry).ok())
//     )

// }

// pub fn query() -> ExternResult<Vec<DeepKeyAgent>> {
//     let root_query = ChainQueryFilter::new()
//         .sequence_range(KEYSET_ROOT_CHAIN_INDEX..KEYSET_ROOT_CHAIN_INDEX+1)
//         .entry_type(entry_type!(KeysetRoot)?)
//         .include_entries(true);
//     let mut deepkey_agents: Vec<DeepKeyAgent> = hdk::prelude::query(root_query)?
//     .iter()
//     .filter_map(|element| match element.entry().to_app_option::<KeysetRoot>() {
//         Ok(Some(keyset_root)) => Some(DeepKeyAgent::Root(keyset_root)),
//         Ok(None) => {
//             error!(?element, ?crate::error::Error::ElementMissingKeysetRoot(element));
//             None
//         },
//         Err(error) => {
//             error!(?element, ?error, ?crate::error::Error::ElementMissingKeysetRoot(element));
//             None
//         },
//     })
//     .collect();

//     if deepkey_agents.len() != 0 {
//         let device_query = ChainQueryFilter::new()
//             .entry_type(entry_type!(DeviceAuthorization)?)
//             .include_entries(true);
//         let mut devices: Vec<DeepKeyAgent> = hdk::prelude::query(device_query)?
//         .iter()
//         .filter_map(|element| match element.entry().to_app_option::<DeviceAuthorization>() {
//             Ok(Some(device_authorization)) => Some(DeepKeyAgent::Device(device_authorization)),
//             Ok(None) => {
//                 error!(?element, "{}", Error::MissingDevice);
//                 None
//             },
//             Err(error) => {
//                 error!(?element, ?error, "{}", Error::MissingDevice);
//                 None
//             },
//         })
//         .collect();
//         deepkey_agents.append(&mut devices);
//     }
//     else {
//         error!("{}", Error::MissingKeysetRoot);
//     }
//     Ok(deepkey_agents)
// }

