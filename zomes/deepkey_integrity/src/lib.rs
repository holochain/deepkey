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
use crate::init::*;
use crate::keyset_root::validate::*;
use crate::device_authorization::device_invite_acceptance::validate::*;
use crate::device_authorization::device_invite::validate::*;
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
        // 
        // The Action Authority (see: holochain/crates/holochain_integrity_types/src/op.rs)
        //
        // This authority is storing the Record's Action.  Validation has access to both the Action
        // and the Entry; any validation that requires other source-chain Records must obtain them
        // from the DHT or a Chain Authority.
        //
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
                        EntryTypes::JoiningProof(joining_proof) =>
                            confirm_action_joining_proof(&action, joining_proof),
                        EntryTypes::KeysetRoot(keyset_root) =>
                            confirm_action_keyset_root(&action, keyset_root),
                        EntryTypes::DeviceInvite(device_invite) =>
                            confirm_action_device_invite(&action, &device_invite),
                        EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) =>
                            confirm_action_device_invite_acceptance(&action, device_invite_acceptance.to_owned()),
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

        // 
        // The Entry Authority
        //
        // This authority is storing the Entry due to some Action.  Doing validation that requires
        // the source-chain will require network access to obtain other Record Action or Entry data
        // from the DHT or a Chain Authority.
        // 
        // Also responsible for Entry Metadata: Register{Update,Delete}, Register{Create,Update}Link
        // 
        OpType::StoreEntry(store_entry) => {
            let action: Action = match op {
                Op::StoreEntry(StoreEntry{ action, .. }) => action.hashed.content.into(),
                _ => unreachable!(),
            };
	    debug!("- Store Entry: {:?}", store_entry );
            match store_entry {
                OpEntry::CreateEntry { entry_hash: _, entry_type } |
                OpEntry::UpdateEntry { entry_hash: _, entry_type, .. } => match entry_type {
                    EntryTypes::JoiningProof(joining_proof) =>
                        confirm_action_joining_proof(&action, joining_proof),
                    EntryTypes::KeysetRoot(keyset_root) =>
                        confirm_action_keyset_root(&action, keyset_root),
                    EntryTypes::DeviceInvite(device_invite) =>
                        confirm_action_device_invite(&action, &device_invite),
                    EntryTypes::DeviceInviteAcceptance(device_invite_acceptance) =>
                        confirm_action_device_invite_acceptance(&action, device_invite_acceptance.to_owned()),
                    _other => {
                        debug!("Storing some other Entry: {:?}", _other);
                        Ok(ValidateCallbackResult::Valid)
                    },
                },
                OpEntry::CreateAgent(_) |
                OpEntry::UpdateAgent { .. } => {
                    Ok(ValidateCallbackResult::Valid)
                }
            }
        },

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


        //
        // The Chain Authority
        // 
        // This authority has the previous items of the chain. here we introduce rules based on
        // previous Actions, with local (immediate) access to the source-chain.  TODO: show an
        // invalidation use-case or explain why we signal valid by default here TODO: could all
        // cases marked with 'todo!()' really happen here as well?
        // 
        OpType::RegisterAgentActivity(agent_activity) => {
            let signed_action: SignedHashed<Action> = match op {
                Op::RegisterAgentActivity(RegisterAgentActivity{ action, .. }) => action,
                _ => unreachable!(),
            };
	    debug!("- Register Agent Activity: {:?}", agent_activity );
            match agent_activity {
                // Agent joining network validation
                OpActivity::AgentValidationPkg(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::CloseChain(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::CreateAgent(_agent_pubkey) => {
                    // TODO: we could perform a check on the new agent's pubkey
                    Ok(ValidateCallbackResult::Valid)
                }
                OpActivity::CreateCapClaim(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::CreateCapGrant(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::UpdateEntry { entry_hash, entry_type, .. } |
                OpActivity::CreateEntry{ entry_hash, entry_type } => match entry_type {
		    // For each entry type, obtain the required ValidationData.  For some, we need
                    // only the current Record (Action + Entry); for others, we need part/all of the
                    // source-chain.  TODO: convert to directly validate w/ internal
                    // must_get_agent_activity calls.
                    Some(UnitEntryTypes::DeviceInvite) =>
                        confirm_chain_device_invite(signed_action),
                    _other => {
                        debug!("Storing some other Action: {:?} == {:?}: {:?}", entry_hash, entry_type, _other);
                        Ok(ValidateCallbackResult::Valid)
                    },
		},
                OpActivity::CreatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::CreateLink { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::DeleteEntry { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::DeleteLink(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::Dna(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::InitZomesComplete => {
                    // we could perform an integrity check on the Zome genesis
                    Ok(ValidateCallbackResult::Valid)
                },
                OpActivity::OpenChain(_) => Ok(ValidateCallbackResult::Valid),
                OpActivity::UpdateAgent { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::UpdateCapClaim { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::UpdateCapGrant { .. } => Ok(ValidateCallbackResult::Valid),
                OpActivity::UpdatePrivateEntry { .. } => Ok(ValidateCallbackResult::Valid),
            }
        }
    }
}
