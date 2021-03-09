use hdk::prelude::*;

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