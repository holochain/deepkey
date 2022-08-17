pub mod change_rule;
pub mod device_authorization;
pub mod dna_binding;
pub mod entry;
pub mod generator;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod key_meta;
pub mod validate;
pub mod error;
pub mod init;

/// Re-export at the root for tests to use entry def macros.
pub use entry::entry_defs;

use hdk::prelude::*;

use crate::change_rule::entry::ChangeRule;
use crate::change_rule::validate::*;
use crate::device_authorization::device_invite_acceptance::{ entry::DeviceInviteAcceptance, validate::* };
use crate::error::*;
use crate::entry::UnitEntryTypes;

/// 
/// Centralized validation.  Breaks out the DHT Ops allowed on various DeepKey Entries
/// 
/// See Op in: ~/src/holochain/crates/holochain_zome_types/src/op.rs
///
pub fn genesis_self_check(data: GenesisSelfCheckData) ->  ExternResult<ValidateCallbackResult> {
    // TODO
    // check data.dna_def
    // check data.membrane_proof
    // check data.agent_key
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // validation::common_validatation(data)
    let info = zome_info()?;
    match op {
        // Validation for elements based on header type
        Op::StoreRecord { element } => {
            match element.header() {
                Action::Dna(_) => todo!(),
                Action::AgentValidationPkg(_) => todo!(),
                Action::InitZomesComplete(_) => todo!(),
                Action::CreateLink(create) => todo!() /* match create.link_type.into() {
                    LinkTypes::Fish => todo!(),
                    _ => {}
                } */ ,
                Action::DeleteLink(_) => todo!(),
                Action::OpenChain(_) => todo!(),
                Action::CloseChain(_) => todo!(),
                Action::Create(create) => match create.entry_type {
                    EntryType::AgentPubKey => todo!(),
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "change_rule" => {
                                let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = create_entry.try_into();
                                match device_invite_acceptance_maybe {
                                    Ok(device_invite_acceptance) => confirm_create_entry_device_invite_acceptance(device_invite_acceptance, create_header),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            "device_invite_acceptance" => {
                                let change_rule_maybe: Result<ChangeRule> = create_entry.try_into();
                                match change_rule_maybe {
                                    Ok(change_rule) => confirm_create_entry_key_change_rule(change_rule, header),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            _ => todo!(),
                        }
                    }
                    EntryType::CapClaim => todo!(),
                    EntryType::CapGrant => todo!(),
                },
                Action::Update(update) => match update.entry_type {
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "change_rule" => {
                                let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = create_entry.try_into();
                                match device_invite_acceptance_maybe {
                                    Ok(device_invite_acceptance) => confirm_update_entry_device_invite_acceptance(device_invite_acceptance, create_header),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            "device_invite_acceptance" => {
                                let change_rule_maybe: Result<ChangeRule> = create_entry.try_into();
                                match change_rule_maybe {
                                    Ok(change_rule) => confirm_update_entry_key_change_rule(change_rule, header),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            _ => todo!(),
                        }
                    }
                },
                Action::Delete(delete) =>  match update.entry_type {
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "change_rule" => {
                                let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = create_entry.try_into();
                                let prev_header: ActionHash = Action::Delete(delete).prev_header();
                                match device_invite_acceptance_maybe {
                                    Ok(device_invite_acceptance) => confirm_delete_entry_device_invite_acceptance(device_invite_acceptance, prev_header, Action::Delete(delete)),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            "device_invite_acceptance" => {
                                let change_rule_maybe: Result<ChangeRule> = create_entry.try_into();
                                let prev_header: ActionHash = Action::Delete(delete).prev_header();
                                match change_rule_maybe {
                                    Ok(change_rule) => confirm_delete_entry_key_change_rule(change_rule, prev_header, Action::Delete(delete)),
                                    Err(err) => Error::EntryMissing.into()
                                }
                            }
                            _ => todo!(),
                        }
                    }
                },
            }
            Ok(ValidateCallbackResult::Valid)
        }
        // Validation for Entries based on Entry::App type
        Op::StoreEntry { entry, header, .. } => {
            /*
            // An Entry::App(_) and HoloHash<EntryCreationHeader>, representing either a Create or
            // Update.  Lets break out the Action, and call confirm_create_entry or
            // confirm_update_entry
            let header: HoloHashed<EntryCreationHeader> = header.into();
            let header: EntryCreationHeader = *header.as_content();
            match header.into() {
                Action::Create(create_header) => confirm_create_entry(entry, Action::Create(create_header)),
                Action::Update(update_header) => confirm_update_entry(entry, Action::Update(update_header)),
            }
            */
            match header.hashed.content.entry_type() {
                //entry_def_index!(String::from("change_rule")) => {
                //entry_def_index!(ChangeRule) => {
                UnitEntryTypes::ChangeRule => {
                    let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = create_entry.try_into();
                    match device_invite_acceptance_maybe {
                        Ok(device_invite_acceptance) => match header.hashed.content {
                            Action::Create(_) => confirm_create_entry_device_invite_acceptance(device_invite_acceptance, header.hashed.content),
                            Action::Update(_) => confirm_update_entry_device_invite_acceptance(device_invite_acceptance, header.hashed.content),
                        },
                        Err(err) => Error::EntryMissing.into()
                    }
                },
                //entry_def_index!(String::from("device_invite_acceptance")) => {
                UnitEntryTypes::DeviceInviteAcceptance => {
                    let change_rule_maybe: Result<ChangeRule> = create_entry.try_into();
                    match change_rule_maybe {
                        Ok(change_rule) => match header.hashed.content {
                            Action::Create(_) => confirm_create_entry_key_change_rule(change_rule, header),
                            Action::Update(_) => confirm_create_entry_key_change_rule(change_rule, header),
                        },
                        Err(err) => Error::EntryMissing.into()
                    }
                },
                _ => todo!(),
            }
        },
        // Agent joining network validation
        // this is a new DHT op
        Op::RegisterAgent { header, agent_pub_key } => {
            // get validation package and then do stuff
            Ok(ValidateCallbackResult::Valid)
        },
        // Chain structure validation
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),

        Op::RegisterCreateLink { create_link: _ } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { create_link: _, .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDelete { delete, original_entry, original_header } => {
            let delete_header: HoloHashed<Action> = delete.into(); // A SignedHashed<Action::Delete>>
            let delete_header: Action = *delete_header.as_content();
            let original_header: HoloHashed<Action> = original_header.into();
            confirm_delete_entry(original_entry, original_header.as_hash(), delete_header)
        }
    }
}

pub fn confirm_create_entry(create_entry: Entry, create_header: Action) -> ExternResult<ValidateCallbackResult> {
    match create_entry {
        Entry::App(_) => {
            let change_rule_maybe: Result<ChangeRule> = create_entry.try_into();
            match change_rule_maybe {
                Ok(change_rule) => return confirm_create_entry_key_change_rule(change_rule, header),
                _ => { },
            }
            let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = create_entry.try_into();
            match device_invite_acceptance_maybe {
                Ok(device_invite_acceptance) => return confirm_create_entry_device_invite_acceptance(device_invite_acceptance, create_header),
                _ => { },
            }
            Error::EntryMissing.into() // TODO: We must handle every known Entry Type in DNA
        }
        // match entry.try_into() {
        //     Ok(ChangeRule { keyset_root, keyset_leaf, spec_change })
        //         => confirm_create_entry_key_change_rule(ChangeRule { keyset_root, keyset_leaf, spec_change }, header),
        //     Ok(DeviceInviteAcceptance { keyset_root, keyset_leaf, spec_change })
        //         => confirm_create_entry_device_invite_acceptance(DeviceInviteAcceptance { keyset_root, keyset_leaf, spec_change }, header),
        //     _ => Ok(ValidateCallbackResult::Valid),  // TODO: Remove; account for *all* Structs
        // },
        // _ => Ok(ValidateCallbackResult::Valid),
    }
}

pub fn confirm_update_entry(update_entry: Entry, update_header: Action) -> ExternResult<ValidateCallbackResult> {
    match update_entry {
        Entry::App(_) => {
            let change_rule_maybe: Result<ChangeRule> = update_entry.try_into();
            match change_rule_maybe {
                Ok(change_rule) => return confirm_update_entry_key_change_rule(change_rule, update_header),
                _ => { }
            }
            let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = update_entry.try_into();
            match device_invite_acceptance_maybe {
                Ok(device_invite_acceptance) => return confirm_update_entry_device_invite_acceptance(device_invite_acceptance, update_header),
                _ => { },
            }
            Error::EntryMissing.into() // TODO: We must handle every known Entry Type in DNA
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}

pub fn confirm_delete_entry(original_entry: Entry, original_header: Action, delete_header: Action) -> ExternResult<ValidateCallbackResult> {
    match original_entry {
        Entry::App(_) => {
            let change_rule_maybe: Result<ChangeRule> = original_entry.try_into();
            match change_rule_maybe {
                Ok(change_rule) => return confirm_delete_entry_key_change_rule(change_rule, original_header, delete_header),
                _ => { },
            }
            let device_invite_acceptance_maybe: Result<DeviceInviteAcceptance> = original_entry.try_into();
            match device_invite_acceptance_maybe {
                Ok(device_invite_acceptance) => return confirm_delete_entry_device_invite_acceptance(device_invite_acceptance, original_header, delete_header),
                _ => { },
            }
            Error::EntryMissing.into() // TODO: We must handle every known Entry Type in DNA
        },
        _ => Ok(ValidateCallbackResult::Valid),
    }
}
