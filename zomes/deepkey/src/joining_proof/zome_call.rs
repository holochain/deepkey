use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::init::{ JoiningProof, KeysetProof, MembraneProof };
use deepkey_integrity::entry::EntryTypes;

///
/// initialize -- Accept an invitation to join a Deepkey Keyset space, or establish a new KeysetRoot
/// 
///     Each Deepkey instance either joins or establishes a Keyset space.
///
///     The JoiningProof commit must be the first Action performed on a Deepkey source-chain (at
/// sequence 4); if establishing a new Keyset space, the KeysetRoot commit (referenced within the
/// JoiningProof) must appear immediately afterwards (at sequence 5).
/// 
#[hdk_extern]
fn initialize((keyset_proof, membrane_proof_maybe): (KeysetProof, Option<MembraneProof>)) -> ExternResult<ActionHash> {
    let membrane_proof = membrane_proof_maybe.unwrap_or( MembraneProof::None );
    debug!("join: w/ {:?}, {:?}", keyset_proof, membrane_proof);
    create_entry(EntryTypes::JoiningProof(JoiningProof{ keyset_proof, membrane_proof }))
}
