use hdi::prelude::*;

// LinkTypes::DeviceName
// This is just a link, with the device's name in the tag field.
// The link doesn't really point anywhere important. This could be added if we find something useful here.

pub fn validate_create_link_device_name(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}

pub fn validate_delete_link_device_name(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "DeviceInviteToDeviceInviteAcceptances links cannot be deleted",
    )))
}
