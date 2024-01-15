use hdi::prelude::*;


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyGeneration {
    pub new_key: AgentPubKey,
    // The private key has signed the deepkey agent key to prove ownership
    pub new_key_signing_of_author: Signature,
    // TODO
    // generator: ActionHash, // This is the key authorized to generate new keys on this chain
    // generator_signature: Signature, // The generator key signing the new key
}
