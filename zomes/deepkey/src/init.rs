use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::error::Error;
use deepkey_integrity::init::JOINING_PROOF_CHAIN_INDEX;
//use deepkey_integrity::init::{ KeysetProof, MembraneProof, JoiningProof };
//use deepkey_integrity::entry::EntryTypes;

/// 
/// init -- establish a new Deepkey Agent source-chain
/// 
///     Ensures that the source-chain is in the empty state, in preparation for an initial
/// JoiningProof.  There is no way to communicate to the Agent at init time, so we cannot commit one
/// here.  Since the JoiningProof validation code will also confirm its exact location in the
/// source-chain, there is no validation to perform here -- just ensure the chain's presently empty.
/// 
#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let q = ChainQueryFilter::new()
//        .sequence_range(ChainQueryFilterRange::ActionSeqRange(JOINING_PROOF_CHAIN_INDEX, JOINING_PROOF_CHAIN_INDEX+1));
        .sequence_range(ChainQueryFilterRange::ActionSeqRange(0, JOINING_PROOF_CHAIN_INDEX+1));
    let maybe_proof: Vec<Record> = query(q)?;
    debug!("Initializing Zome: w/ existing proof: {:?}", maybe_proof);
    if maybe_proof.len() > JOINING_PROOF_CHAIN_INDEX as usize {
        debug!("Initializing failed w/ {} entries perhaps already containing proof(s): {:?}", maybe_proof.len(), maybe_proof);
        Err(Error::MultipleJoinProof.into())
    } else {
        debug!("Initializing Zome: successful");
        Ok(InitCallbackResult::Pass)
    }
        
    /*
     *
    let joining_proof = if maybe_proof.len() >= JOINING_PROOF_CHAIN_INDEX as usize {
        match JoiningProof::try_from(&maybe_proof[JOINING_PROOF_CHAIN_INDEX as usize]) {
            Ok(jp) => jp,
            Err(e) => {
                debug!("Initializing failed; No JoiningProof at source-chain index {}: {:?}", JOINING_PROOF_CHAIN_INDEX, e);
                return Err(e.into())
            }
        }
    }
    else {
        debug!("Initializing failed w/ {} proof(s)", maybe_proof.len());
        return Error::MultipleJoinProof.into()
    };

    match joining_proof.keyset_proof {
        KeysetProof::KeysetRoot(keyset_root) => create_entry(EntryTypes::KeysetRoot(keyset_root))?,
        KeysetProof::DeviceInviteAcceptance(device_invite_acceptance) => create_entry(EntryTypes::DeviceInviteAcceptance(device_invite_acceptance))?,
    };

    // @todo
    match joining_proof.membrane_proof {
        MembraneProof::None => { },
        MembraneProof::ProofOfWork(ref _proof_of_work) => { },
        MembraneProof::ProofOfStake(ref _proof_of_stake) => { },
        MembraneProof::ProofOfAuthority(ref _proof_of_authority) => { },
    }
    debug!("Initializing Zome: successful w/ commit of: {:?}", joining_proof.membrane_proof);
    Ok(InitCallbackResult::Pass)
    *
    */
}
