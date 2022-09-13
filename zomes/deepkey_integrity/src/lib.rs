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

pub mod validate_classic; // classic validate_... function ValidateData struct

/// Re-export at the root for tests to use entry def macros, coordinator zome.
pub use entry::entry_defs;
pub use hdk;

use hdk::prelude::*;
//use crate::validate_classic::*;

//use crate::change_rule::entry::ChangeRule;
//use crate::change_rule::validate::*;
use crate::device_authorization::device_invite_acceptance::validate::*;
use crate::init::*;
use crate::entry::{LinkTypes, EntryTypes, UnitEntryTypes};

/// 
/// Centralized validation.  Breaks out the DHT Ops allowed on various DeepKey Entries
/// 
/// See Op in: ~/src/holochain/crates/holochain_zome_types/src/op.rs
///
#[hdk_extern]
pub fn genesis_self_check(_data: GenesisSelfCheckData) ->  ExternResult<ValidateCallbackResult> {
    // TODO
    // check data.dna_def
    // check data.membrane_proof
    // check data.agent_key
    debug!("Genesis Self Check for {:?}: DNA {:?} ({:?}) w/ Zomes: {:?}",
           _data.agent_key, _data.dna_info.name, _data.dna_info.hash, _data.dna_info.zome_names );
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // validation::common_validatation(data)
    let info = zome_info()?;
    debug!("Validating {:?} Zome Op: {:?}", info.name.0, op );
    match op.to_type::<EntryTypes, _>()? {
        // This authority is storing the Record (Action + Entry)
        OpType::StoreRecord(store_record) => {
            let action = match op {
                Op::StoreRecord(StoreRecord{ record }) => record.action().to_owned(),
                _ => unreachable!(),
            };
	    debug!("- Store Record: Action: {:?} == {:?}", action, store_record);
            match store_record {
                OpRecord::Dna(_dna_hash) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::AgentValidationPkg(_membrane) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::InitZomesComplete =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::OpenChain(_dna_hash) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CloseChain(_dna_hash) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapClaim(_entry_hash) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateCapGrant(_entry_hash) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapClaim{ entry_hash: _, original_action_hash: _, original_entry_hash: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateCapGrant{ entry_hash: _, original_action_hash: _, original_entry_hash: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateEntry{ entry_hash: _, entry_type } => {
                    match entry_type {
                        EntryTypes::JoiningProof(joining_proof)
                            => confirm_create_action_joining_proof(&action, joining_proof),
                        _ => Ok(ValidateCallbackResult::Valid),
                    }
                }
                OpRecord::UpdateEntry{ entry_hash: _, original_action_hash: _, original_entry_hash: _, entry_type: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::DeleteEntry{ original_action_hash: _, original_entry_hash: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreatePrivateEntry { entry_hash: _, entry_type: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdatePrivateEntry { entry_hash: _, original_action_hash: _, original_entry_hash: _, entry_type: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateAgent(_agent) =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::UpdateAgent{ original_key: _, new_key: _, original_action_hash: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::CreateLink{ base_address: _, target_address: _, tag: _, link_type: _ } =>
                    Ok(ValidateCallbackResult::Valid),
                OpRecord::DeleteLink(_link_action_hash) =>
                    Ok(ValidateCallbackResult::Valid),
            }
        },
        // This authority is storing the Entry due to some Action, but don't have local access to
        // the Action
        OpType::StoreEntry(store_entry) => {
	    debug!("- Store Entry: {:?}", store_entry );
            match store_entry {
                OpEntry::CreateEntry {
                    entry_hash,
                    entry_type,
                } => match entry_type {
                    EntryTypes::JoiningProof(joining_proof) => {
                        debug!("Storing JoiningProof Entry: {:?}", joining_proof);
                        Ok(ValidateCallbackResult::Valid)
                    },
                    EntryTypes::DeviceInviteAcceptance(ref device_invite_acceptance) => {
                        debug!("Storing DeviceInviteAcceptance Entry: {:?} == {:?}", entry_hash, entry_type);
                        confirm_create_entry_device_invite_acceptance(device_invite_acceptance.to_owned(), op.author().to_owned())
                    },
                    _other => {
                        debug!("Storing some other Entry: {:?}", _other);
                        Ok(ValidateCallbackResult::Valid)
                    },
                },
                OpEntry::UpdateEntry {
                    entry_hash,
                    entry_type,
                    ..
                } => match entry_type {
                    EntryTypes::JoiningProof(joining_proof) => {
                        debug!("Updating JoiningProof Entry: {:?}", joining_proof);
                        Ok(ValidateCallbackResult::Valid)
                    },
                    EntryTypes::DeviceInviteAcceptance(ref device_invite_acceptance) => {
                        debug!("Updating DeviceInviteAcceptance Entry: {:?} == {:?}", entry_hash, entry_type);
                        confirm_update_entry_device_invite_acceptance(device_invite_acceptance.to_owned())
                    },
                    _other => {
                        debug!("Updating some other Entry: {:?}", _other);
                        Ok(ValidateCallbackResult::Valid)
                    },
                },
                OpEntry::CreateAgent(_) | OpEntry::UpdateAgent { .. } => {
                    Ok(ValidateCallbackResult::Valid)
                }
            }
        },

        // This authority has the previous items of the chain. here we introduce rules based on
        // previous Actions, with local (immediate) access to the source-chain.  TODO: show an
        // invalidation use-case or explain why we signal valid by default here TODO: could all
        // cases marked with 'todo!()' really happen here as well?
        OpType::RegisterAgentActivity(agent_activity) => {
	    debug!("- Register Agent Activity: {:?}", agent_activity );
            match agent_activity {
                // Agent joining network validation
                OpActivity::AgentValidationPkg(_) => todo!(),
                OpActivity::CloseChain(_) => todo!(),
                OpActivity::CreateAgent(_agent_pubkey) => {
                    // TODO: we could perform a check on the new agent's pubkey
                }
                OpActivity::CreateCapClaim(_) => todo!(),
                OpActivity::CreateCapGrant(_) => todo!(),
                OpActivity::CreateEntry{ entry_hash, entry_type } => match entry_type {
		    // For each entry type, obtain the required ValidationData.  For some, we need
                    // only the current Record (Action + Entry); for others, we need part/all of the
                    // source-chain.  TODO: convert to directly validate w/ internal
                    // must_get_agent_activity calls.
                    Some(UnitEntryTypes::JoiningProof) => {
                        // let agent_activity: Vec<RegisterAgentActivity> = vec![];
                        // let validate_data: ValidateData = (entry_type, agent_activity).into();
                        debug!("Storing JoiningProof Action: {:?} == {:?}", entry_hash, entry_type);
                    },
                    Some(UnitEntryTypes::DeviceInviteAcceptance) => {
                        debug!("Storing DeviceInviteAcceptance Action: {:?} == {:?}", entry_hash, entry_type);
                    },
                    _other => {
                        debug!("Storing some other Action: {:?} == {:?}: {:?}", entry_hash, entry_type, _other);
                    },
		},
                OpActivity::CreatePrivateEntry { .. } => todo!(),
                OpActivity::CreateLink { .. } => todo!(),
                OpActivity::DeleteEntry { .. } => todo!(),
                OpActivity::DeleteLink(_) => todo!(),
                OpActivity::Dna(_) => todo!(),
                OpActivity::InitZomesComplete => {
                    // we could perform an integrity check on the Zome genesis
                },
                OpActivity::OpenChain(_) => todo!(),
                OpActivity::UpdateAgent { .. } => todo!(),
                OpActivity::UpdateCapClaim { .. } => todo!(),
                OpActivity::UpdateCapGrant { .. } => todo!(),
                OpActivity::UpdateEntry { .. } => todo!(),
                OpActivity::UpdatePrivateEntry { .. } => todo!(),
            }

            Ok(ValidateCallbackResult::Valid)
        }

        // Validation for creating links
        OpType::RegisterCreateLink {
            link_type,
            base_address,
            target_address,
            tag,
            ..
        } => {
	    debug!("- Register Create Link: {:?} w/ tag: {:?} from {:?} -> {:?}",
                   link_type, tag, base_address, target_address );
            match link_type {
                LinkTypes::AgentInvite => Ok(ValidateCallbackResult::Valid),
                LinkTypes::AgentInviteNotify => Ok(ValidateCallbackResult::Valid),
            }
        },

        // Validation for deleting links
        OpType::RegisterDeleteLink {
            link_type,
            // original_link_hash,
            base_address,
            target_address,
            tag,
            ..
        } => {
	    debug!("- Register Delete Link: {:?} w/ tag: {:?} from {:?} -> {:?}",
                   link_type, tag, base_address, target_address );
            match link_type {
                LinkTypes::AgentInvite => Ok(ValidateCallbackResult::Invalid(
                    "AgentInvite Link cannot be deleted".to_string())),
                LinkTypes::AgentInviteNotify => Ok(ValidateCallbackResult::Valid),
            }
        },

        OpType::RegisterUpdate(update_entry) => {
	    debug!("- Register Update: {:?}", update_entry );
            match update_entry {
                OpUpdate::Entry {
                    entry_hash,
                    original_action_hash: _,
                    original_entry_hash,
                    original_entry_type,
                    new_entry_type,
                } => match new_entry_type {
                    EntryTypes::JoiningProof(ref _joining_proof) => {
                        debug!("Register JoiningProof Update: from {:?} == {:?}, into: {:?} == {:?}",
                               original_entry_hash, original_entry_type,
                               entry_hash, new_entry_type,
                        );
			Ok(ValidateCallbackResult::Valid)
                    },
                    _ => Ok(ValidateCallbackResult::Valid),
                },
                OpUpdate::PrivateEntry {
                    entry_hash: _,
                    original_action_hash: _,
                    original_entry_hash: _,
                    original_entry_type: _,
                    new_entry_type: _,
                } => todo!(),
                OpUpdate::Agent {
                    new_key: _,
                    original_key: _,
                    original_action_hash: _,
                } => todo!(),
                OpUpdate::CapClaim {
                    entry_hash: _,
                    original_action_hash: _,
                    original_entry_hash: _,
                } => todo!(),
                OpUpdate::CapGrant {
                    entry_hash: _,
                    original_action_hash: _,
                    original_entry_hash: _,
                } => todo!(),
            }
        },

        OpType::RegisterDelete(delete_entry) => {
	    debug!("- Register Delete: {:?}", delete_entry );
            Ok(ValidateCallbackResult::Invalid(
                "deleting entries isn't valid".to_string(),
            ))
        },
    }


    /*
    match op {
        // Validation for elements based on header type
        Op::StoreRecord { element } => {
            match element.header() {
                Action::Dna(_) => todo!(),
                Action::AgentValidationPkg(_) => todo!(),
                Action::InitZomesComplete(_) => todo!(),
                Action::CreateLink(create) => todo!()
                //  match create.link_type.into() {
                //     LinkTypes::Fish => todo!(),
                //     _ => {}
                // },
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
                _ => Error::EntryMissing.into() // TODO: We must handle every known Entry Type in DNA
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
    */
}

/*
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
*/
