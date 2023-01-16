use hdk::prelude::*;

use deepkey_integrity::{
    device_invite_acceptance::*, EntryTypes, JoiningProof, KeysetProof, MembraneProof,
};

#[hdk_extern]
pub fn accept_invite(invite_acceptance: DeviceInviteAcceptance) -> ExternResult<ActionHash> {
    let joining_proof = JoiningProof::new(
        KeysetProof::DeviceInviteAcceptance(invite_acceptance.clone()),
        MembraneProof::None,
    );
    let joining_proof_hash = create_entry(EntryTypes::JoiningProof(joining_proof))?;

    Ok(joining_proof_hash)
}
