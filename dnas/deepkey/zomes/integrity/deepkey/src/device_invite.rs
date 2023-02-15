use hdi::prelude::*;

use crate::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub struct DeviceInvite {
    pub keyset_root: ActionHash,
    // Either the KeysetRoot or the DeviceInviteAcceptance
    pub parent: ActionHash,
    pub invitee: AgentPubKey,
}

impl DeviceInvite {
    pub fn new(keyset_root: ActionHash, parent: ActionHash, invitee: AgentPubKey) -> Self {
        Self {
            keyset_root,
            parent,
            invitee,
        }
    }
}

pub fn validate_create_device_invite(
    _action: EntryCreationAction,
    device_invite: DeviceInvite,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record(device_invite.keyset_root.clone())?;
    let _keyset_root: crate::KeysetRoot = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Dependant action must be accompanied by an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_device_invite(
    _action: Update,
    _device_invite: DeviceInvite,
    _original_action: EntryCreationAction,
    _original_device_invite: DeviceInvite,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Device Invites cannot be updated",
    )))
}
pub fn validate_delete_device_invite(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_device_invite: DeviceInvite,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Device Invites cannot be deleted",
    )))
}
pub fn validate_create_link_keyset_root_to_device_invites(
    _action: CreateLink,
    base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(base_address);
    let record = must_get_valid_record(action_hash)?;
    let _keyset_root: crate::KeysetRoot = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _device_invite: crate::DeviceInvite = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_keyset_root_to_device_invites(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "KeysetRootToDeviceInvites links cannot be deleted",
    )))
}
pub fn validate_create_link_invitee_to_device_invites(
    _action: CreateLink,
    _base_address: AnyLinkableHash,
    target_address: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    let action_hash = ActionHash::from(target_address);
    let record = must_get_valid_record(action_hash)?;
    let _device_invite: crate::DeviceInvite = record
        .entry()
        .to_app_option()
        .map_err(|e| wasm_error!(e))?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Linked action must reference an entry"
        ))))?;
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_delete_link_invitee_to_device_invites(
    _action: DeleteLink,
    _original_action: CreateLink,
    _base: AnyLinkableHash,
    _target: AnyLinkableHash,
    _tag: LinkTag,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "InviteeToDeviceInvites links cannot be deleted",
    )))
}
/// TODO: Run this as a validation
pub fn validate_device_invite_original(
    invite: DeviceInvite,
    invite_create_action: Create,
) -> ExternResult<ValidateCallbackResult> {
    // // A DeviceInvite must deserialize cleanly from the validating record.
    // let invite_action_hash = invite_acceptance.invite;
    // let invite_record = must_get_valid_record(invite_action_hash.clone())?;

    // let invite_option = invite_record
    //     .entry()
    //     .to_app_option::<DeviceInvite>()
    //     .ok()
    //     .flatten();
    // if let None = invite_option {
    //     return Ok(ValidateCallbackResult::Invalid(
    //         "DeviceInviteAcceptance contains an invalid DeviceInvite".into(),
    //     ));
    // }
    // let invite = invite_option.unwrap();

    // The KSR must be fetched and deserialized into a KeysetRoot.
    // let ksr_action_hash = invite_acceptance.keyset_root_authority;
    let ksr_action_hash = invite.keyset_root;
    let ksr_record = must_get_valid_record(ksr_action_hash.clone())?;
    let ksr_option = KeysetRoot::try_from(ksr_record.clone()).ok();
    if let None = ksr_option {
        return Ok(ValidateCallbackResult::Invalid(
            "DeviceInviteAcceptance contains a malformed KeysetRoot".into(),
        ));
    }
    let ksr = ksr_option.unwrap();

    let parent_record = must_get_valid_record(invite.parent.clone())?;

    let invitor = parent_record.action().author().clone();

    // An invitee must have a different agent pubkey than the invitor
    if invite.invitee == invitor {
        return Ok(ValidateCallbackResult::Invalid(
            "An invitee must have a different agent pubkey than the invitor".into(),
        ));
    }

    // If the author of the invitation is the FDA in the invitation's KSR
    if invitor == ksr.first_deepkey_agent {
        if *ksr_record.action().author() != invitor {
            return Ok(ValidateCallbackResult::Invalid(
                "Invitor claims to be Keyset Root Authority, but the Keyset Root is written on another chain.".into(),
            ));
        }
        let invite_action_hash = hash_action(invite_create_action.into())?;

        // Do a hash-bounded query from the invite hash back to the KSR in the invitor's source chain.
        let filter = ChainFilter::new(invite_action_hash).until(ksr_action_hash.clone());
        let activities = must_get_agent_activity(invitor, filter)?;

        // Check that that range contains no invite acceptances (have abandoned the Keyset they are inviting a new device into).
        let dia_def = AppEntryDef::try_from(UnitEntryTypes::DeviceInviteAcceptance).unwrap();
        for activity in activities.into_iter() {
            if let Some(EntryType::App(app_entry_def)) = activity.action.action().entry_type() {
                if *app_entry_def == dia_def {
                    return Ok(ValidateCallbackResult::Invalid(
                        "The invitor has abandoned the Keyset they are inviting a new device into"
                            .into(),
                    ));
                }
            };
        }
    } else {
        // Search from invite backwards & find the first `DeviceInviteAcceptance` in their chain.
        let filter = ChainFilter::new(invite.parent);
        let activities = must_get_agent_activity(invitor, filter)?;
        let invitor_dias = activities
            .into_iter()
            .filter_map(|activity| {
                if let Some(EntryType::App(app_entry_def)) = activity.action.action().entry_type() {
                    if *app_entry_def
                        == AppEntryDef::try_from(UnitEntryTypes::DeviceInviteAcceptance).unwrap()
                    {
                        return Some(activity.clone());
                    }
                };
                None
            })
            .collect::<Vec<_>>();
        if invitor_dias.is_empty() {
            return Ok(ValidateCallbackResult::Invalid(
                "The invitor is inviting into a Keyset they do not have authority over".into(),
            ));
        }
        // The invite in that `DeviceInviteAcceptance` must fetch and deserialize to a `DeviceInvite`.
        let invitor_dia_action = invitor_dias.first().unwrap().action.action();
        let invitor_dia_entry = must_get_entry(invitor_dia_action.entry_hash().unwrap().clone())?;
        let invitor_dia = DeviceInviteAcceptance::try_from(invitor_dia_entry).unwrap();
        let invitors_original_invite_action_hash = invitor_dia.invite;
        let invitors_original_invite_record =
            must_get_valid_record(invitors_original_invite_action_hash.clone())?;
        let invite_option = invitors_original_invite_record
            .entry()
            .to_app_option::<DeviceInvite>()
            .ok()
            .flatten();
        if let None = invite_option {
            return Ok(ValidateCallbackResult::Invalid(
                "Invitor's DeviceInviteAcceptance contains an invalid DeviceInvite".into(),
            ));
        }
        // That deserialized `DeviceInvite` must have the same KSR authority as the new `DeviceInvite` currently being validated.
        let invite = invite_option.unwrap();
        if invite.keyset_root != ksr_action_hash {
            return Ok(ValidateCallbackResult::Invalid(
                "Invitor is part of a different KeysetRoot Authority than the one it is inviting into".into(),
            ));
        }
        // Also in that `DeviceInvite`, the invitee must be the author of the new `DeviceInvite`.
        if invite.invitee != *parent_record.action().author() {
            return Ok(ValidateCallbackResult::Invalid(
                "Inviting Agent is different from the invitation it is using as proof of authority"
                    .into(),
            ));
        }
    }

    // The invite in that acceptance must fetch and deserialize to a DeviceInvite.
    // That deserialized DeviceInvite must have the same KSR as the new DeviceInvite currently being validated.
    // Also in that DeviceInvite, the invitee must be the author of the new DeviceInvite.
    Ok(ValidateCallbackResult::Valid)
}


