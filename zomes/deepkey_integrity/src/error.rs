use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Record missing its Entry")]
    EntryMissing,

    #[error("Wrong header for a Record")]
    WrongHeader,

    #[error("Keyset sequence is wrong")]
    KeysetSeq,

    #[error("KeyRegistration sequence is wrong")]
    KeyRegistrationSeq,

    #[error("KeysetRoot not followed by ChangeRule")]
    KeysetRootThenChangeRule,

    #[error("More than one JoiningProof found")]
    MultipleJoinProof,

    #[error("Attempted to update a JoiningProof")]
    UpdateJoiningProof,

    #[error("Attempted to delete a JoiningProof")]
    DeleteJoiningProof,

    #[error("JoiningProof created in the wrong position")]
    JoiningProofPosition,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
    }
}

impl From<Error> for InitCallbackResult {
    fn from(e: Error) -> Self {
        InitCallbackResult::Fail(e.to_string())
    }
}

impl From<Error> for WasmError {
    fn from(e: Error) -> Self {
        wasm_error!(WasmErrorInner::Guest(e.to_string()))
    }
}

impl From<Error> for ExternResult<ValidateCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}

impl From<Error> for ExternResult<InitCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}
