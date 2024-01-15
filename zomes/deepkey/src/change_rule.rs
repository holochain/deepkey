use holo_hash;
use hdi::prelude::*;

use crate::{
    error::Error,
    Authorization,
    AuthorizedSpecChange,
};


// The author needs to be linked from the KeysetRoot
#[hdk_entry_helper]
#[derive(Clone)]
pub struct ChangeRule {
    pub keyset_root: ActionHash,
    pub keyset_leaf: ActionHash,
    pub spec_change: AuthorizedSpecChange,
}

impl ChangeRule {
    pub fn new(
        keyset_root: ActionHash,
        keyset_leaf: ActionHash,
        spec_change: AuthorizedSpecChange,
    ) -> Self {
        Self {
            keyset_root,
            keyset_leaf,
            spec_change,
        }
    }

    pub fn authorize(&self, authorization: &[Authorization], data: &[u8]) -> Result<(), Error> {
        if authorization.len() != self.spec_change.new_spec.sigs_required as usize {
            Err(Error::WrongNumberOfSignatures)
        } else {
            for (position, signature) in authorization.iter() {
                match self
                    .spec_change
                    .new_spec
                    .authorized_signers
                    .get(*position as usize)
                {
                    Some(agent) => {
                        if !verify_signature_raw(
                            holo_hash::AgentPubKey::from_raw_32( agent.to_vec() ),
                            signature.to_owned(),
                            data.to_vec(),
                        )? {
                            // Short circuit any failed sig.
                            return Err(Error::BadUpdateSignature);
                        }
                    }
                    None => return Err(Error::AuthorizedPositionOutOfBounds),
                }
            }
            Ok(())
        }
    }
}
