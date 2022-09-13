use hdk::prelude::*;
use crate::error::Error;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::validate_classic::*;

// The joining proof is added to the chain immediately after init (only Dna, AgentValidation,
// AgentPubKey and InitZomesComplete)
pub const JOINING_PROOF_CHAIN_INDEX: u32 = 4;

// @todo - e.g. configurable difficulty over hashing the DNA - https://docs.rs/pow/0.2.0/pow/
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfWork([u8; 32]);

// @todo
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfStake([u8; 32]);

// @todo
#[derive(Debug, Serialize, Deserialize)]
pub struct ProofOfAuthority([u8; 32]);

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize)]
pub enum KeysetProof {
    KeysetRoot(KeysetRoot),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

//#[hdk_entry(id = "joining_proof")]
#[hdk_entry_helper]
pub struct JoiningProof {
    pub keyset_proof: KeysetProof,
    pub membrane_proof: MembraneProof,
}
/* 
 * TODO: How do we allow all CRUD?
 * 
impl TryFrom<&Record> for JoiningProof {
    type Error = crate::error::Error;
    fn try_from(element: &Record) -> Result<Self, Self::Error> {
        match element.action() {
            // Only
            Action::Create(_) | Action::Update(_) | Action::Delete(_) => {
                Ok(match element.entry() {
                    RecordEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}
 */

#[hdk_extern]
fn validate_create_entry_joining_proof(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    match JoiningProof::try_from(&validate_data.element) {
        Ok(joining_proof) => confirm_action_joining_proof( validate_data.element.action(), joining_proof ),
        Err(e) => Ok(ValidateCallbackResult::Invalid(e.to_string())),
    }
}
// 
// confirm_ -- Validate based on the role of the agent
// 
// 
// 

pub fn confirm_action_joining_proof( action: &Action, joining_proof: JoiningProof ) -> ExternResult<ValidateCallbackResult> {
    debug!(" -- Confirm {:?}: {:?}", action, joining_proof);
    match action {
        Action::Create(_create_action) =>
            if action.action_seq() == JOINING_PROOF_CHAIN_INDEX as u32 {
                Ok(ValidateCallbackResult::Valid)
            } else {
                Error::JoiningProofPosition.into()
            },
        Action::Update(_) => Error::UpdateJoiningProof.into(),
        Action::Delete(_) => Error::DeleteJoiningProof.into(),
        _ => Ok(ValidateCallbackResult::Invalid(format!("Invalid Action for JoiningProof: {:?}", action ))),
    }
}
