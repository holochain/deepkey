pub mod key_anchor;
pub use key_anchor::*;
pub mod joining_proof;
pub use joining_proof::*;
pub mod key_registration;
pub use key_registration::*;
pub mod key_revocation;
pub use key_revocation::*;
pub mod key_generation;
pub use key_generation::*;
pub mod device_invite_acceptance;
pub use device_invite_acceptance::*;
pub mod device_invite;
pub use device_invite::*;
pub mod change_rule;
pub use change_rule::*;
pub mod authorized_spec_change;
pub use authorized_spec_change::*;
pub mod authority_spec;
pub use authority_spec::*;
pub use keyset_root::*;
pub mod error;
pub mod keyset_root;
pub use error::*;
pub mod source_of_authority;
use hdi::prelude::*;
pub use source_of_authority::*;
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    KeysetRoot(KeysetRoot),
    AuthoritySpec(AuthoritySpec),
    AuthorizedSpecChange(AuthorizedSpecChange),
    ChangeRule(ChangeRule),
    DeviceInvite(DeviceInvite),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
    // JoiningProof(JoiningProof),
    KeyGeneration(KeyGeneration),
    KeyRevocation(KeyRevocation),
    KeyRegistration(KeyRegistration),
    KeyAnchor(KeyAnchor),
}
#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    SignerToAuthoritySpecs,
    ChangeRuleUpdates,
    KeysetRootToDeviceInvites,
    InviteeToDeviceInvites,
    DeviceInviteToDeviceInviteAcceptances,
}
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_agent_joining(
    _agent_pub_key: AgentPubKey,
    _membrane_proof: &Option<MembraneProof>,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreEntry(store_entry) => match store_entry {
            OpEntry::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::KeysetRoot(keyset_root) => {
                    validate_create_keyset_root(EntryCreationAction::Create(action), keyset_root)
                }
                EntryTypes::AuthoritySpec(authority_spec) => validate_create_authority_spec(
                    EntryCreationAction::Create(action),
                    authority_spec,
                ),
                EntryTypes::AuthorizedSpecChange(authorized_spec_change) => {
                    validate_create_authorized_spec_change(
                        EntryCreationAction::Create(action),
                        authorized_spec_change,
                    )
                }
                EntryTypes::ChangeRule(change_rule) => {
                    validate_create_change_rule(EntryCreationAction::Create(action), change_rule)
                }
                EntryTypes::DeviceInvite(device_invite) => validate_create_device_invite(
                    EntryCreationAction::Create(action),
                    device_invite,
                ),
                EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) => {
                    validate_create_device_invite_acceptance(
                        EntryCreationAction::Create(action),
                        device_invite_acceptance,
                    )
                }
                EntryTypes::KeyGeneration(key_generation) => validate_create_key_generation(
                    EntryCreationAction::Create(action),
                    key_generation,
                ),
                EntryTypes::KeyRevocation(key_revocation) => validate_create_key_revocation(
                    EntryCreationAction::Create(action),
                    key_revocation,
                ),
                EntryTypes::KeyRegistration(key_registration) => validate_create_key_registration(
                    EntryCreationAction::Create(action),
                    key_registration,
                ),
                EntryTypes::KeyAnchor(key_anchor) => {
                    validate_create_key_anchor(EntryCreationAction::Create(action), key_anchor)
                }
            },
            OpEntry::UpdateEntry {
                app_entry, action, ..
            } => match app_entry {
                EntryTypes::KeysetRoot(keyset_root) => {
                    validate_create_keyset_root(EntryCreationAction::Update(action), keyset_root)
                }
                EntryTypes::AuthoritySpec(authority_spec) => validate_create_authority_spec(
                    EntryCreationAction::Update(action),
                    authority_spec,
                ),
                EntryTypes::AuthorizedSpecChange(authorized_spec_change) => {
                    validate_create_authorized_spec_change(
                        EntryCreationAction::Update(action),
                        authorized_spec_change,
                    )
                }
                EntryTypes::ChangeRule(change_rule) => {
                    validate_create_change_rule(EntryCreationAction::Update(action), change_rule)
                }
                EntryTypes::DeviceInvite(device_invite) => validate_create_device_invite(
                    EntryCreationAction::Update(action),
                    device_invite,
                ),
                EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) => {
                    validate_create_device_invite_acceptance(
                        EntryCreationAction::Update(action),
                        device_invite_acceptance,
                    )
                }
                EntryTypes::KeyGeneration(key_generation) => validate_create_key_generation(
                    EntryCreationAction::Update(action),
                    key_generation,
                ),
                EntryTypes::KeyRevocation(key_revocation) => validate_create_key_revocation(
                    EntryCreationAction::Update(action),
                    key_revocation,
                ),
                EntryTypes::KeyRegistration(key_registration) => validate_create_key_registration(
                    EntryCreationAction::Update(action),
                    key_registration,
                ),
                EntryTypes::KeyAnchor(key_anchor) => {
                    validate_create_key_anchor(EntryCreationAction::Update(action), key_anchor)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterUpdate(update_entry) => match update_entry {
            OpUpdate::Entry {
                original_action,
                original_app_entry,
                app_entry,
                action,
            } => match (app_entry, original_app_entry) {
                (EntryTypes::KeyAnchor(key_anchor), EntryTypes::KeyAnchor(original_key_anchor)) => {
                    validate_update_key_anchor(
                        action,
                        key_anchor,
                        original_action,
                        original_key_anchor,
                    )
                }
                (
                    EntryTypes::KeyRegistration(key_registration),
                    EntryTypes::KeyRegistration(original_key_registration),
                ) => validate_update_key_registration(
                    action,
                    key_registration,
                    original_action,
                    original_key_registration,
                ),
                (
                    EntryTypes::KeyRevocation(key_revocation),
                    EntryTypes::KeyRevocation(original_key_revocation),
                ) => validate_update_key_revocation(
                    action,
                    key_revocation,
                    original_action,
                    original_key_revocation,
                ),
                (
                    EntryTypes::KeyGeneration(key_generation),
                    EntryTypes::KeyGeneration(original_key_generation),
                ) => validate_update_key_generation(
                    action,
                    key_generation,
                    original_action,
                    original_key_generation,
                ),
                (
                    EntryTypes::DeviceInviteAcceptance(device_invite_acceptance),
                    EntryTypes::DeviceInviteAcceptance(original_device_invite_acceptance),
                ) => validate_update_device_invite_acceptance(
                    action,
                    device_invite_acceptance,
                    original_action,
                    original_device_invite_acceptance,
                ),
                (
                    EntryTypes::DeviceInvite(device_invite),
                    EntryTypes::DeviceInvite(original_device_invite),
                ) => validate_update_device_invite(
                    action,
                    device_invite,
                    original_action,
                    original_device_invite,
                ),
                (
                    EntryTypes::ChangeRule(change_rule),
                    EntryTypes::ChangeRule(original_change_rule),
                ) => validate_update_change_rule(
                    action,
                    change_rule,
                    original_action,
                    original_change_rule,
                ),
                (
                    EntryTypes::AuthorizedSpecChange(authorized_spec_change),
                    EntryTypes::AuthorizedSpecChange(original_authorized_spec_change),
                ) => validate_update_authorized_spec_change(
                    action,
                    authorized_spec_change,
                    original_action,
                    original_authorized_spec_change,
                ),
                (
                    EntryTypes::AuthoritySpec(authority_spec),
                    EntryTypes::AuthoritySpec(original_authority_spec),
                ) => validate_update_authority_spec(
                    action,
                    authority_spec,
                    original_action,
                    original_authority_spec,
                ),
                (
                    EntryTypes::KeysetRoot(keyset_root),
                    EntryTypes::KeysetRoot(original_keyset_root),
                ) => validate_update_keyset_root(
                    action,
                    keyset_root,
                    original_action,
                    original_keyset_root,
                ),
                _ => Ok(ValidateCallbackResult::Invalid(
                    "Original and updated entry types must be the same".to_string(),
                )),
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDelete(delete_entry) => match delete_entry {
            OpDelete::Entry {
                original_action,
                original_app_entry,
                action,
            } => match original_app_entry {
                EntryTypes::KeysetRoot(keyset_root) => {
                    validate_delete_keyset_root(action, original_action, keyset_root)
                }
                EntryTypes::AuthoritySpec(authority_spec) => {
                    validate_delete_authority_spec(action, original_action, authority_spec)
                }
                EntryTypes::AuthorizedSpecChange(authorized_spec_change) => {
                    validate_delete_authorized_spec_change(
                        action,
                        original_action,
                        authorized_spec_change,
                    )
                }
                EntryTypes::ChangeRule(change_rule) => {
                    validate_delete_change_rule(action, original_action, change_rule)
                }
                EntryTypes::DeviceInvite(device_invite) => {
                    validate_delete_device_invite(action, original_action, device_invite)
                }
                EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) => {
                    validate_delete_device_invite_acceptance(
                        action,
                        original_action,
                        device_invite_acceptance,
                    )
                }
                EntryTypes::KeyGeneration(key_generation) => {
                    validate_delete_key_generation(action, original_action, key_generation)
                }
                EntryTypes::KeyRevocation(key_revocation) => {
                    validate_delete_key_revocation(action, original_action, key_revocation)
                }
                EntryTypes::KeyRegistration(key_registration) => {
                    validate_delete_key_registration(action, original_action, key_registration)
                }
                EntryTypes::KeyAnchor(key_anchor) => {
                    validate_delete_key_anchor(action, original_action, key_anchor)
                }
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterCreateLink {
            link_type,
            base_address: _,
            target_address: _,
            tag: _,
            action: _,
        } => match link_type {
            // LinkTypes::SignerToAuthoritySpecs => validate_create_link_signer_to_authority_specs(
            //     action,
            //     base_address,
            //     target_address,
            //     tag,
            // ),
            // LinkTypes::ChangeRuleUpdates => {
            //     validate_create_link_change_rule_updates(action, base_address, target_address, tag)
            // }
            // LinkTypes::KeysetRootToDeviceInvites => {
            //     validate_create_link_keyset_root_to_device_invites(
            //         action,
            //         base_address,
            //         target_address,
            //         tag,
            //     )
            // }
            // LinkTypes::InviteeToDeviceInvites => validate_create_link_invitee_to_device_invites(
            //     action,
            //     base_address,
            //     target_address,
            //     tag,
            // ),
            // LinkTypes::DeviceInviteToDeviceInviteAcceptances => {
            //     validate_create_link_device_invite_to_device_invite_acceptances(
            //         action,
            //         base_address,
            //         target_address,
            //         tag,
            //     )
            // }
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterDeleteLink {
            link_type,
            base_address,
            target_address,
            tag,
            original_action,
            action,
        } => match link_type {
            LinkTypes::SignerToAuthoritySpecs => validate_delete_link_signer_to_authority_specs(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::ChangeRuleUpdates => validate_delete_link_change_rule_updates(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::KeysetRootToDeviceInvites => {
                validate_delete_link_keyset_root_to_device_invites(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
            LinkTypes::InviteeToDeviceInvites => validate_delete_link_invitee_to_device_invites(
                action,
                original_action,
                base_address,
                target_address,
                tag,
            ),
            LinkTypes::DeviceInviteToDeviceInviteAcceptances => {
                validate_delete_link_device_invite_to_device_invite_acceptances(
                    action,
                    original_action,
                    base_address,
                    target_address,
                    tag,
                )
            }
        },
        FlatOp::StoreRecord(store_record) => match store_record {
            OpRecord::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::KeysetRoot(keyset_root) => {
                    validate_create_keyset_root(EntryCreationAction::Create(action), keyset_root)
                }
                EntryTypes::AuthoritySpec(authority_spec) => validate_create_authority_spec(
                    EntryCreationAction::Create(action),
                    authority_spec,
                ),
                EntryTypes::AuthorizedSpecChange(authorized_spec_change) => {
                    validate_create_authorized_spec_change(
                        EntryCreationAction::Create(action),
                        authorized_spec_change,
                    )
                }
                EntryTypes::ChangeRule(change_rule) => {
                    validate_create_change_rule(EntryCreationAction::Create(action), change_rule)
                }
                EntryTypes::DeviceInvite(device_invite) => validate_create_device_invite(
                    EntryCreationAction::Create(action),
                    device_invite,
                ),
                EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) => {
                    validate_create_device_invite_acceptance(
                        EntryCreationAction::Create(action),
                        device_invite_acceptance,
                    )
                }
                EntryTypes::KeyGeneration(key_generation) => validate_create_key_generation(
                    EntryCreationAction::Create(action),
                    key_generation,
                ),
                EntryTypes::KeyRevocation(key_revocation) => validate_create_key_revocation(
                    EntryCreationAction::Create(action),
                    key_revocation,
                ),
                EntryTypes::KeyRegistration(key_registration) => validate_create_key_registration(
                    EntryCreationAction::Create(action),
                    key_registration,
                ),
                EntryTypes::KeyAnchor(key_anchor) => {
                    validate_create_key_anchor(EntryCreationAction::Create(action), key_anchor)
                }
            },
            OpRecord::UpdateEntry {
                original_action_hash,
                app_entry,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for an update must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                match app_entry {
                    EntryTypes::KeysetRoot(keyset_root) => {
                        let result = validate_create_keyset_root(
                            EntryCreationAction::Update(action.clone()),
                            keyset_root.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_keyset_root: Option<KeysetRoot> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_keyset_root = match original_keyset_root {
                                Some(keyset_root) => keyset_root,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_keyset_root(
                                action,
                                keyset_root,
                                original_action,
                                original_keyset_root,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::AuthoritySpec(authority_spec) => {
                        let result = validate_create_authority_spec(
                            EntryCreationAction::Update(action.clone()),
                            authority_spec.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_authority_spec: Option<AuthoritySpec> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_authority_spec = match original_authority_spec {
                                Some(authority_spec) => authority_spec,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_authority_spec(
                                action,
                                authority_spec,
                                original_action,
                                original_authority_spec,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::AuthorizedSpecChange(authorized_spec_change) => {
                        let result = validate_create_authorized_spec_change(
                            EntryCreationAction::Update(action.clone()),
                            authorized_spec_change.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_authorized_spec_change: Option<AuthorizedSpecChange> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_authorized_spec_change =
                                match original_authorized_spec_change {
                                    Some(authorized_spec_change) => authorized_spec_change,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                            validate_update_authorized_spec_change(
                                action,
                                authorized_spec_change,
                                original_action,
                                original_authorized_spec_change,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::ChangeRule(change_rule) => {
                        let result = validate_create_change_rule(
                            EntryCreationAction::Update(action.clone()),
                            change_rule.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_change_rule: Option<ChangeRule> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_change_rule = match original_change_rule {
                                Some(change_rule) => change_rule,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_change_rule(
                                action,
                                change_rule,
                                original_action,
                                original_change_rule,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::DeviceInvite(device_invite) => {
                        let result = validate_create_device_invite(
                            EntryCreationAction::Update(action.clone()),
                            device_invite.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_device_invite: Option<DeviceInvite> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_device_invite = match original_device_invite {
                                Some(device_invite) => device_invite,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_device_invite(
                                action,
                                device_invite,
                                original_action,
                                original_device_invite,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) => {
                        let result = validate_create_device_invite_acceptance(
                            EntryCreationAction::Update(action.clone()),
                            device_invite_acceptance.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_device_invite_acceptance: Option<DeviceInviteAcceptance> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_device_invite_acceptance =
                                match original_device_invite_acceptance {
                                    Some(device_invite_acceptance) => device_invite_acceptance,
                                    None => {
                                        return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                    }
                                };
                            validate_update_device_invite_acceptance(
                                action,
                                device_invite_acceptance,
                                original_action,
                                original_device_invite_acceptance,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::KeyGeneration(key_generation) => {
                        let result = validate_create_key_generation(
                            EntryCreationAction::Update(action.clone()),
                            key_generation.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_key_generation: Option<KeyGeneration> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_key_generation = match original_key_generation {
                                Some(key_generation) => key_generation,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_key_generation(
                                action,
                                key_generation,
                                original_action,
                                original_key_generation,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::KeyRevocation(key_revocation) => {
                        let result = validate_create_key_revocation(
                            EntryCreationAction::Update(action.clone()),
                            key_revocation.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_key_revocation: Option<KeyRevocation> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_key_revocation = match original_key_revocation {
                                Some(key_revocation) => key_revocation,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_key_revocation(
                                action,
                                key_revocation,
                                original_action,
                                original_key_revocation,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::KeyRegistration(key_registration) => {
                        let result = validate_create_key_registration(
                            EntryCreationAction::Update(action.clone()),
                            key_registration.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_key_registration: Option<KeyRegistration> =
                                original_record
                                    .entry()
                                    .to_app_option()
                                    .map_err(|e| wasm_error!(e))?;
                            let original_key_registration = match original_key_registration {
                                Some(key_registration) => key_registration,
                                None => {
                                    return Ok(
                                            ValidateCallbackResult::Invalid(
                                                "The updated entry type must be the same as the original entry type"
                                                    .to_string(),
                                            ),
                                        );
                                }
                            };
                            validate_update_key_registration(
                                action,
                                key_registration,
                                original_action,
                                original_key_registration,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                    EntryTypes::KeyAnchor(key_anchor) => {
                        let result = validate_create_key_anchor(
                            EntryCreationAction::Update(action.clone()),
                            key_anchor.clone(),
                        )?;
                        if let ValidateCallbackResult::Valid = result {
                            let original_key_anchor: Option<KeyAnchor> = original_record
                                .entry()
                                .to_app_option()
                                .map_err(|e| wasm_error!(e))?;
                            let original_key_anchor = match original_key_anchor {
                                Some(key_anchor) => key_anchor,
                                None => {
                                    return Ok(
                                        ValidateCallbackResult::Invalid(
                                            "The updated entry type must be the same as the original entry type"
                                                .to_string(),
                                        ),
                                    );
                                }
                            };
                            validate_update_key_anchor(
                                action,
                                key_anchor,
                                original_action,
                                original_key_anchor,
                            )
                        } else {
                            Ok(result)
                        }
                    }
                }
            }
            OpRecord::DeleteEntry {
                original_action_hash,
                action,
                ..
            } => {
                let original_record = must_get_valid_record(original_action_hash)?;
                let original_action = original_record.action().clone();
                let original_action = match original_action {
                    Action::Create(create) => EntryCreationAction::Create(create),
                    Action::Update(update) => EntryCreationAction::Update(update),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "Original action for a delete must be a Create or Update action"
                                .to_string(),
                        ));
                    }
                };
                let app_entry_type = match original_action.entry_type() {
                    EntryType::App(app_entry_type) => app_entry_type,
                    _ => {
                        return Ok(ValidateCallbackResult::Valid);
                    }
                };
                let entry = match original_record.entry().as_option() {
                    Some(entry) => entry,
                    None => {
                        if original_action.entry_type().visibility().is_public() {
                            return Ok(
                                    ValidateCallbackResult::Invalid(
                                        "Original record for a delete of a public entry must contain an entry"
                                            .to_string(),
                                    ),
                                );
                        } else {
                            return Ok(ValidateCallbackResult::Valid);
                        }
                    }
                };
                let original_app_entry = match EntryTypes::deserialize_from_type(
                    app_entry_type.zome_index.clone(),
                    app_entry_type.entry_index.clone(),
                    &entry,
                )? {
                    Some(app_entry) => app_entry,
                    None => {
                        return Ok(
                                ValidateCallbackResult::Invalid(
                                    "Original app entry must be one of the defined entry types for this zome"
                                        .to_string(),
                                ),
                            );
                    }
                };
                match original_app_entry {
                    EntryTypes::KeysetRoot(original_keyset_root) => {
                        validate_delete_keyset_root(action, original_action, original_keyset_root)
                    }
                    EntryTypes::AuthoritySpec(original_authority_spec) => {
                        validate_delete_authority_spec(
                            action,
                            original_action,
                            original_authority_spec,
                        )
                    }
                    EntryTypes::AuthorizedSpecChange(original_authorized_spec_change) => {
                        validate_delete_authorized_spec_change(
                            action,
                            original_action,
                            original_authorized_spec_change,
                        )
                    }
                    EntryTypes::ChangeRule(original_change_rule) => {
                        validate_delete_change_rule(action, original_action, original_change_rule)
                    }
                    EntryTypes::DeviceInvite(original_device_invite) => {
                        validate_delete_device_invite(
                            action,
                            original_action,
                            original_device_invite,
                        )
                    }
                    EntryTypes::DeviceInviteAcceptance(original_device_invite_acceptance) => {
                        validate_delete_device_invite_acceptance(
                            action,
                            original_action,
                            original_device_invite_acceptance,
                        )
                    }
                    EntryTypes::KeyGeneration(original_key_generation) => {
                        validate_delete_key_generation(
                            action,
                            original_action,
                            original_key_generation,
                        )
                    }
                    EntryTypes::KeyRevocation(original_key_revocation) => {
                        validate_delete_key_revocation(
                            action,
                            original_action,
                            original_key_revocation,
                        )
                    }
                    EntryTypes::KeyRegistration(original_key_registration) => {
                        validate_delete_key_registration(
                            action,
                            original_action,
                            original_key_registration,
                        )
                    }
                    EntryTypes::KeyAnchor(original_key_anchor) => {
                        validate_delete_key_anchor(action, original_action, original_key_anchor)
                    }
                }
            }
            OpRecord::CreateLink {
                base_address: _,
                target_address: _,
                tag: _,
                link_type,
                action: _,
            } => match link_type {
                // LinkTypes::SignerToAuthoritySpecs => {
                //     validate_create_link_signer_to_authority_specs(
                //         action,
                //         base_address,
                //         target_address,
                //         tag,
                //     )
                // }
                // LinkTypes::ChangeRuleUpdates => validate_create_link_change_rule_updates(
                //     action,
                //     base_address,
                //     target_address,
                //     tag,
                // ),
                // LinkTypes::KeysetRootToDeviceInvites => {
                //     validate_create_link_keyset_root_to_device_invites(
                //         action,
                //         base_address,
                //         target_address,
                //         tag,
                //     )
                // }
                // LinkTypes::InviteeToDeviceInvites => {
                //     validate_create_link_invitee_to_device_invites(
                //         action,
                //         base_address,
                //         target_address,
                //         tag,
                //     )
                // }
                // LinkTypes::DeviceInviteToDeviceInviteAcceptances => {
                //     validate_create_link_device_invite_to_device_invite_acceptances(
                //         action,
                //         base_address,
                //         target_address,
                //         tag,
                //     )
                // }
                _ => Ok(ValidateCallbackResult::Valid),
            },
            OpRecord::DeleteLink {
                original_action_hash,
                base_address,
                action,
            } => {
                let record = must_get_valid_record(original_action_hash)?;
                let create_link = match record.action() {
                    Action::CreateLink(create_link) => create_link.clone(),
                    _ => {
                        return Ok(ValidateCallbackResult::Invalid(
                            "The action that a DeleteLink deletes must be a CreateLink".to_string(),
                        ));
                    }
                };
                let link_type = match LinkTypes::from_type(
                    create_link.zome_index.clone(),
                    create_link.link_type.clone(),
                )? {
                    Some(lt) => lt,
                    None => {
                        return Ok(ValidateCallbackResult::Valid);
                    }
                };
                match link_type {
                    LinkTypes::SignerToAuthoritySpecs => {
                        validate_delete_link_signer_to_authority_specs(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::ChangeRuleUpdates => validate_delete_link_change_rule_updates(
                        action,
                        create_link.clone(),
                        base_address,
                        create_link.target_address,
                        create_link.tag,
                    ),
                    LinkTypes::KeysetRootToDeviceInvites => {
                        validate_delete_link_keyset_root_to_device_invites(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::InviteeToDeviceInvites => {
                        validate_delete_link_invitee_to_device_invites(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                    LinkTypes::DeviceInviteToDeviceInviteAcceptances => {
                        validate_delete_link_device_invite_to_device_invite_acceptances(
                            action,
                            create_link.clone(),
                            base_address,
                            create_link.target_address,
                            create_link.tag,
                        )
                    }
                }
            }
            OpRecord::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CreateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::Dna { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::OpenChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::CloseChain { .. } => Ok(ValidateCallbackResult::Valid),
            OpRecord::InitZomesComplete { .. } => Ok(ValidateCallbackResult::Valid),
            _ => Ok(ValidateCallbackResult::Valid),
        },
        FlatOp::RegisterAgentActivity(agent_activity) => match agent_activity {
            OpActivity::CreateAgent { agent, action } => {
                let previous_action = must_get_action(action.prev_action)?;
                match previous_action.action() {
                        Action::AgentValidationPkg(
                            AgentValidationPkg { membrane_proof, .. },
                        ) => validate_agent_joining(agent, membrane_proof),
                        _ => {
                            Ok(
                                ValidateCallbackResult::Invalid(
                                    "The previous action for a `CreateAgent` action must be an `AgentValidationPkg`"
                                        .to_string(),
                                ),
                            )
                        }
                    }
            }
            _ => Ok(ValidateCallbackResult::Valid),
        },
    }
}
