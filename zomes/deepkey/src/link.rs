pub mod error;

use hdk::prelude::*;
use holochain_types::prelude::HOLO_HASH_FULL_LEN;
use crate::keyset_root::link::KEYSET_ROOT_TO_AGENT_TAG;
use crate::keyset_root::link::validate_create_link_keyset_root_to_agent;
use crate::link::error::Error;
use crate::keyset_root::link::KEYSET_ROOT_TO_AGENT_TAG_LEN;

/// The agent tag is the KEYSET_ROOT_TO_AGENT_TAG length + an AgentPubKey.
pub const AGENT_TAG_FULL_LEN: usize = HOLO_HASH_FULL_LEN + KEYSET_ROOT_TO_AGENT_TAG_LEN;

pub struct AgentLinkTag([u8; AGENT_TAG_FULL_LEN]);

secure_primitive!(AgentLinkTag, AGENT_TAG_FULL_LEN);

impl TryFrom<LinkTag> for AgentLinkTag {
    type Error = Error;
    fn try_from(link_tag: LinkTag) -> Result<Self, Self::Error> {
        let mut link_tag_bytes = link_tag.into_inner();
        let agent_bytes = link_tag_bytes.split_off(KEYSET_ROOT_TO_AGENT_TAG_LEN);
        if link_tag_bytes == vec![KEYSET_ROOT_TO_AGENT_TAG] {
            Ok(AgentLinkTag::try_from(agent_bytes).map_err(|_| Error::NotAgentTag)?)
        }
        else {
            Err(Error::NotAgentTag)
        }
    }
}

impl From<AgentLinkTag> for LinkTag {
    fn from(agent_link_tag: AgentLinkTag) -> LinkTag {
        let mut link_tag_bytes = vec![KEYSET_ROOT_TO_AGENT_TAG];
        link_tag_bytes.append(&mut agent_link_tag.as_ref().to_vec());
        LinkTag::from(agent_link_tag)
    }
}

impl From<AgentLinkTag> for AgentPubKey {
    fn from(agent_link_tag: AgentLinkTag) -> Self {
        AgentPubKey::from_raw_36(agent_link_tag.as_ref()[KEYSET_ROOT_TO_AGENT_TAG_LEN..].to_vec())
    }
}

impl From<AgentPubKey> for AgentLinkTag {
    fn from(agent_pub_key: AgentPubKey) -> Self {
        let mut bytes = [0; AGENT_TAG_FULL_LEN];
        bytes.clone_from_slice(agent_pub_key.as_ref());
        Self(bytes)
    }
}

pub fn link_tag_to_agent(link_tag: LinkTag) -> ExternResult<AgentPubKey> {
    Ok(AgentPubKey::try_from(AgentLinkTag::try_from(link_tag)?)?)
}

pub fn agent_to_link_tag(agent_pub_key: AgentPubKey) -> LinkTag {
    LinkTag::from(AgentLinkTag::from(agent_pub_key))
}

#[hdk_extern]
pub fn validate_create_link(create_link_data: ValidateCreateLinkData) -> ExternResult<ValidateLinkCallbackResult> {
    let tag_byte: u8 = create_link_data.link_add.tag.0[0];

    match tag_byte {
        KEYSET_ROOT_TO_AGENT_TAG => validate_create_link_keyset_root_to_agent(create_link_data),
        _ => Ok(ValidateLinkCallbackResult::Invalid("Invalid link tag".to_string())),
    }
}

#[hdk_extern]
pub fn validate_delete_link(_: ValidateDeleteLinkData) -> ExternResult<ValidateLinkCallbackResult> {
    Ok(ValidateLinkCallbackResult::Invalid("Links cannot be deleted".to_string()))
}