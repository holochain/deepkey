use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element missing its ChangeRule")]
    EntryMissing,

    #[error("Attempted to delete a ChangeRule")]
    DeleteAttempted,

    #[error("Attempted to update a ChangeRule")]
    UpdateAttempted,

    #[error("The ChangeRule author is not the FDA on the KeysetRoot")]
    AuthorNotFda,

    #[error("Multiple creation signatures found")]
    MultipleCreateSignatures,

    #[error("No creation signature found")]
    NoCreateSignature,

    #[error("Invalid creation signature")]
    BadCreateSignature,

    #[error("The new ChangeRule has a different KeysetRoot")]
    KeysetRootMismatch,

    #[error("The new ChangeRule has the wrong number of signatures")]
    WrongNumberOfSignatures,

    #[error("The new ChangeRule referenced an authorizor position that doesn't exist")]
    AuthorizedPositionOutOfBounds,

    #[error("The new ChangeRule has an invalid signature")]
    BadUpdateSignature,

    #[error("The new ChangeRule has fewer authorized signers than the minimum required signatures")]
    NotEnoughSigners,

    #[error("The new ChangeRule requires zero signatures")]
    NotEnoughSignatures,

    #[error("The new ChangeRule has the same spec as the previous one")]
    IdenticalUpdate,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
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