pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod keyset_root;

use change_rule::*;
use device_invite::{validate_device_invite, DeviceInvite};
use device_invite_acceptance::DeviceInviteAcceptance;
use hdi::prelude::*;
use keyset_root::*;
// use device_invite::*;

// @todo - e.g. configurable difficulty over hashing the DNA - https://docs.rs/pow/0.2.0/pow/
#[hdk_entry_helper]
pub struct ProofOfWork([u8; 32]);

// @todo
#[hdk_entry_helper]
pub struct ProofOfStake([u8; 32]);

// @todo
#[hdk_entry_helper]
pub struct ProofOfAuthority([u8; 32]);

#[hdk_entry_helper]
pub enum MembraneProof {
    // No additional membrane.
    None,
    // Proof of Work membrane.
    ProofOfWork(ProofOfWork),
    // Proof of Stake membrane.
    ProofOfStake(ProofOfStake),
    // Proof of Authority membrane.
    ProofOfAuthority(ProofOfAuthority),
}

#[hdk_entry_helper]
pub enum KeysetProof {
    KeysetRoot(KeysetRoot),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_entry_helper]
pub struct JoiningProof {
    keyset_proof: KeysetProof,
    membrane_proof: MembraneProof,
}

impl JoiningProof {
    pub fn new(keyset_proof: KeysetProof, membrane_proof: MembraneProof) -> Self {
        Self {
            keyset_proof,
            membrane_proof,
        }
    }
}

/*
joining proof smuggles information
can pass in a  device invite acceptance
keyset root (agentkey)
*/
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def]
    JoiningProof(JoiningProof),
    #[entry_def]
    KeysetRoot(KeysetRoot),
    #[entry_def]
    ChangeRule(ChangeRule),
    #[entry_def]
    DeviceInvite(DeviceInvite),
    #[entry_def]
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentToMembraneProof,
    AgentPubKeyToDeviceInvite,
}

////////////////////////////////////////////////////////////////////////////////
// Genesis self-check callback
////////////////////////////////////////////////////////////////////////////////
// #[hdk_extern]
// pub fn genesis_self_check(data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
//     is_membrane_proof_valid(data.agent_key, data.membrane_proof)
// }

////////////////////////////////////////////////////////////////////////////////
// Validation callback
////////////////////////////////////////////////////////////////////////////////

pub fn validate(op: Op) -> ExternResult<ValidateCallbackResult> {
    // return Ok(ValidateCallbackResult::Valid);
    // return Ok(ValidateCallbackResult::Invalid("Invalid operation".into()));
    // return Err(wasm_error!("Invalid operation"));
    match op.to_type::<EntryTypes, LinkTypes>()? {
        OpType::StoreRecord(op_record) => match op_record {
            OpRecord::CreateEntry { app_entry, action } => match app_entry {
                EntryTypes::DeviceInvite(invite) => validate_device_invite(invite, action),
                _ => Ok(ValidateCallbackResult::Valid),
            },
            _ => Ok(ValidateCallbackResult::Valid),
        },
        OpType::RegisterAgentActivity(_) => Ok(ValidateCallbackResult::Valid),
        _ => Ok(ValidateCallbackResult::Valid),
    }
}
