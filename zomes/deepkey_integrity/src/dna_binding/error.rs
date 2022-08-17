use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum Error {
    #[error("Record missing its DnaBinding")]
    EntryMissing,

    #[error("Attempted to delete a DnaBinding")]
    DeleteAttempted,

    #[error("Attempted to update a DnaBinding")]
    UpdateAttempted,

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
