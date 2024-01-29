mod create_entry;
mod update_entry;
mod delete_entry;
mod create_link;
mod delete_link;

// use crate::key_anchor::*;
// use crate::key_registration::*;
// use crate::device_invite_acceptance::*;
// use crate::device_invite::*;
// use crate::change_rule::*;
// use crate::keyset_root::*;
// use crate::device_name::*;
// use crate::key_meta::*;
// use crate::app_binding::*;
use crate::{
    EntryTypes,
    LinkTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};
use hdk::prelude::debug;


#[hdk_extern]
pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    let result = match op.flattened::<EntryTypes, LinkTypes>()? {
        FlatOp::StoreRecord(op_record) => match op_record {
            OpRecord::CreateEntry { app_entry, action } =>
                create_entry::validation( app_entry, action ),
            OpRecord::UpdateEntry { app_entry, action, original_action_hash, original_entry_hash } =>
                update_entry::validation( app_entry, action, original_action_hash, original_entry_hash ),
            OpRecord::DeleteEntry { original_action_hash, original_entry_hash, action } =>
                delete_entry::validation( original_action_hash, original_entry_hash, action ),
            OpRecord::CreateLink { base_address, target_address, tag, link_type, action } =>
                create_link::validation( base_address, target_address, link_type, tag, action ),
            OpRecord::DeleteLink { original_action_hash, base_address, action } =>
                delete_link::validation( original_action_hash, base_address, action ),
            // OpRecord::CreateAgent { agent, action: create },
            // OpRecord::UpdateAgent { original_key, new_key, original_action_hash, action: update },
            // OpRecord::CreateCapClaim { action: create },
            // OpRecord::CreateCapGrant { action: create },
            // OpRecord::CreatePrivateEntry { app_entry_type, action: create },
            // OpRecord::UpdatePrivateEntry { original_action_hash, original_entry_hash, app_entry_type, action: update },
            // OpRecord::UpdateCapClaim { original_action_hash, original_entry_hash, action: update },
            // OpRecord::UpdateCapGrant { original_action_hash, original_entry_hash, action: update },
            // OpRecord::Dna { dna_hash, action: dna },
            // OpRecord::OpenChain { previous_dna_hash, action: open_chain },
            // OpRecord::CloseChain { new_dna_hash, action: close_chain },
            OpRecord::AgentValidationPkg { membrane_proof, action: agent_validation_pkg } => {
                debug!("AgentValidationPkg => {:#?}", agent_validation_pkg );
                debug!("AgentValidationPkg => Membrane proof: {:#?}", membrane_proof );
                invalid!("No agent validation".to_string())
                // valid!()
            },
            // OpRecord::InitZomesComplete { action: init_zomes_complete },
            _ => valid!(),
        },
        // FlatOp::StoreEntry(op_entry),
        FlatOp::RegisterAgentActivity(_op_activity) => {
            // debug!("RegisterAgentActivity => {:#?}", op_activity );
            valid!()
        },
        // FlatOp::RegisterCreateLink { base_address, target_address, tag, link_type, action: create_link },
        // FlatOp::RegisterDeleteLink { original_action, base_address, target_address, tag, link_type, action: delete_link },
        // FlatOp::RegisterUpdate(op_update),
        // FlatOp::RegisterDelete(op_delete),
        _ => valid!(),
    };

    if let Err(WasmError{ error: WasmErrorInner::Guest(msg), .. }) = result {
        invalid!(msg)
    }

    result
}
