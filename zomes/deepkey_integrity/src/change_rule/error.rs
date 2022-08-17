use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Record missing its ChangeRule")]
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

    #[error("The new ChangeRule references a KeysetLeaf that is incompatible with its KeysetRoot")]
    BadKeysetLeaf,

    #[error("The new ChangeRule references a stale keyset leaf")]
    StaleKeysetLeaf,

    #[error("The new ChangeRule has no validation package")]
    MissingValidationPackage,

    #[error("The new ChangeRule has an invalid signature")]
    BadUpdateSignature,

    #[error("The new ChangeRule has fewer authorized signers than the minimum required signatures")]
    NotEnoughSigners,

    #[error("The new ChangeRule requires zero signatures")]
    NotEnoughSignatures,

    #[error("The new ChangeRule update does not reference the root ChangeRule")]
    BranchingUpdates,

    #[error("The ChangeRule created does not immediately follow its KeysetRoot")]
    CreateNotAfterKeysetRoot,

    #[error("The ChangeRule element has the wrong header")]
    WrongHeader,

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

impl From<WasmError> for Error {
    fn from(e: WasmError) -> Error {
        Error::Wasm(e)
    }
}
