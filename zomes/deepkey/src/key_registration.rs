use hdi::prelude::*;

use crate::{
    KeyGeneration,
    KeyRevocation,
};


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub enum KeyRegistration {
    // Creates a key under management of current KSR on this chain
    Create(KeyGeneration),

    // Unmanaged key. Keys for hosted web users may be of this type, cannot replace/revoke
    CreateOnly(KeyGeneration),

    // Revokes a key and replaces it with a newly generated one
    Update(KeyRevocation, KeyGeneration),

    // Permanently revokes a key (Note: still uses an update action.)
    Delete(KeyRevocation)
}
