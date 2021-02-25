use hdk::prelude::*;
use crate::keyset_root::link::KEYSET_ROOT_TO_AGENT_TAG;
use crate::keyset_root::link::validate_create_link_keyset_root_to_agent;

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