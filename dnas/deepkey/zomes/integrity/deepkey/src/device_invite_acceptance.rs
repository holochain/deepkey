use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInviteAcceptance {
    /// The KSRA for the invite being accepted.
    /// Not strictly required for validation as this is on the DeviceInvite.
    /// This is here as it may save network hops other than during.
    pub keyset_root_authority: ActionHash,
    pub invite: ActionHash,
}

impl DeviceInviteAcceptance {
    pub fn new(keyset_root_authority: ActionHash, invite: ActionHash) -> Self {
        Self {
            keyset_root_authority,
            invite,
        }
    }
}

pub fn validate_create_device_invite_acceptance(
    _action: EntryCreationAction,
    device_invite_acceptance: DeviceInviteAcceptance,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(device_invite_acceptance.invite.clone())?;
    let _device_invite: crate::DeviceInvite = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_device_invite_acceptance(
    _action: Update,
    _device_invite_acceptance: DeviceInviteAcceptance,
    _original_action: EntryCreationAction,
    _original_device_invite_acceptance: DeviceInviteAcceptance,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Device Invite Acceptances cannot be updated",
    )))
}
pub fn validate_delete_device_invite_acceptance(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_device_invite_acceptance: DeviceInviteAcceptance,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Device Invite Acceptances cannot be deleted",
    )))
}
pub fn validate_create_link_device_invite_to_device_invite_acceptances(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _device_invite: crate::DeviceInvite = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    // Check the entry type for the given action hash
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _device_invite_acceptance: crate::DeviceInviteAcceptance = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    // TODO: add the appropriate validation rules
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_device_invite_to_device_invite_acceptances(
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
