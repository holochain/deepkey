use hdi::prelude::*;

use crate::SourceOfAuthority;

// @todo - e.g. configurable difficulty over hashing the DNA - https://docs.rs/pow/0.2.0/pow/
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ProofOfWork([u8; 32]);

// @todo
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ProofOfStake([u8; 32]);

// @todo
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ProofOfExternalAuthority([u8; 32]);

#[hdk_entry_helper]
#[derive(Clone)]
pub enum MembraneProof {
    // No additional membrane.
    None,
    // Proof of Work membrane.
    ProofOfWork(ProofOfWork),
    // Proof of Stake membrane.
    ProofOfStake(ProofOfStake),
    // Proof of Authority membrane.
    ProofOfExternalAuthority(ProofOfExternalAuthority),
}

#[hdk_entry_helper]
#[derive(Clone)]
pub struct JoiningProof {
    pub source_of_authority: SourceOfAuthority,
    pub membrane_proof: MembraneProof,
}
pub fn validate_create_joining_proof(
    _action: EntryCreationAction,
    _joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_joining_proof(
    _action: Update,
    _joining_proof: JoiningProof,
    _original_action: EntryCreationAction,
    _original_joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Joining Proofs cannot be updated",
    )))
}
pub fn validate_delete_joining_proof(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_joining_proof: JoiningProof,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(String::from(
        "Joining Proofs cannot be deleted",
    )))
}
