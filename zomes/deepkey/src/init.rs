use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::error::Error;
use deepkey_integrity::init::JOINING_PROOF_CHAIN_INDEX;
use deepkey_integrity::init::{ KeysetProof, MembraneProof, JoiningProof };
use deepkey_integrity::entry::EntryTypes;

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let q = ChainQueryFilter::new()
        .sequence_range(ChainQueryFilterRange::ActionSeqRange(JOINING_PROOF_CHAIN_INDEX, JOINING_PROOF_CHAIN_INDEX+1));
    let maybe_proof: Vec<Record> = query(q)?;

    let joining_proof = if maybe_proof.len() == 1 {
        JoiningProof::try_from(&maybe_proof[0])?
    }
    else {
        return Error::MultipleJoinProof.into()
    };

    match joining_proof.keyset_proof {
        KeysetProof::KeysetRoot(keyset_root) => create_entry(EntryTypes::KeysetRoot(keyset_root))?,
        KeysetProof::DeviceInviteAcceptance(device_invite_acceptance) => create_entry(EntryTypes::DeviceInviteAcceptance(device_invite_acceptance))?,
    };

    // @todo
    match joining_proof.membrane_proof {
        MembraneProof::None => { },
        MembraneProof::ProofOfWork(_proof_of_work) => { },
        MembraneProof::ProofOfStake(_proof_of_stake) => { },
        MembraneProof::ProofOfAuthority(_proof_of_authority) => { },
    }

    Ok(InitCallbackResult::Pass)
}
