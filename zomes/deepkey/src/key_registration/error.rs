use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong Author for a KeyRegistration")]
    BadAuthor,

    #[error("Wrong KeyRegistration variant for this validation op")]
    BadOp,

    #[error("KeyRegistration referenced wrong prior HeaderHash as per Element Header")]
    BadHeaderRef,

    #[error("Bad signature for key generation on KeyRegistration")]
    BadGeneratorSignature,

    #[error("Attempted to revoke a revoke of a KeyRegistration")]
    Tombstone,

    #[error("Attempted to register a key under an agent that was not signed for")]
    BadSelfSignature,
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
    }
}

impl From<Error> for ExternResult<ValidateCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}