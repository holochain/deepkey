use hdk::prelude::*;

// @todo
pub struct ProofOfWork([u8; 32]);

// @todo
pub struct Invite([u8; 32]);

// @todo
pub struct StakeProof([u8; 32]);

// Should we have a membrane for deepkey?
enum JoiningProofData {
    ProofOfWork(ProofOfWork),
    Invite(Invite),
    StakeProof(StakeProof),
}
pub struct JoiningProof {
    device_authorization: DeviceAuthorization,
    additional_data: JoiningProofData
}

#[hdk_extern]
fn init(_: ()) -> ExternResult<InitCallbackResult> {
    let q = ChainQuery::new()
        .range[2..3];
    let maybe_proofs: Vec<Element> = query(q)?;
    // ..

    let joining_proof = JoiningProof::try_from(serialized_proof)?;

    // Get what kind of validation we should be doing from properites.
    zome_info!(..)

}