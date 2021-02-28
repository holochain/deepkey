use hdk::prelude::*;
// use crate::keyset_root::entry;
// use crate::deepkey_agent::DeepKeyAgent;
// use crate::device_authorization::entry::DeviceAuthorization;

/// The tag for links from keyset roots to agents.
/// MUST be globally (to the happ) unique.
pub const KEYSET_ROOT_TO_AGENT_TAG: u8 = 0;

// impl entry::KeysetRoot {
//     pub fn link_to_deepkey_agent(&self, target: DeepKeyAgent) -> ExternResult<HeaderHash> {
//         match target {
//             DeepKeyAgent::Device(device_authorization) => {
//                 create_link(hash_entry(self)?, hash_entry(device_authorization)?, vec![KEYSET_ROOT_TO_AGENT_TAG])
//             },
//             DeepKeyAgent::Root(_) => Err(WasmError::Guest("Attempted to link to a KeysetRoot from a KeysetRoot.".to_string())),
//         }
//     }
// }

pub fn validate_create_link_keyset_root_to_agent(_create_link_data: ValidateCreateLinkData) -> ExternResult<ValidateLinkCallbackResult> {
    // Author needs to be a deepkey agent.
    // let deepkey_agents = crate::deepkey_agent::query()?;
    // let maybe_deepkey_agent = deepkey_agents.iter().find(|deepkey_agent| deepkey_agent.is_agent(&create_link_data.link_add.author));

    // Ok(match maybe_deepkey_agent {
    //     Some(_) => {
    //         // Base needs to be a KeysetRoot.
    //         match entry::KeysetRoot::try_from(create_link_data.base) {
    //             // Target must be a DeviceAuthorization
    //             Ok(_) => match DeviceAuthorization::try_from(create_link_data.target) {
    //                 Ok(_) => ValidateLinkCallbackResult::Valid,
    //                 _ => ValidateLinkCallbackResult::Invalid("Target of keyset root link is not a DeepKeyAgent.".to_string()),
    //             },
    //             _ => ValidateLinkCallbackResult::Invalid("Base of keyset root link is not a KeysetRoot.".to_string()),
    //         }
    //     },
    //     _ => ValidateLinkCallbackResult::Invalid("Author of keyset root link is not a DeepKeyAgent.".to_string()),
    // })
    Ok(ValidateLinkCallbackResult::Valid)
}