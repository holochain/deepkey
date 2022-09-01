use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Generator ChangeRule has wrong author")]
    ChangeRuleAuthor,

    #[error("Attempted to update a Generator")]
    UpdateAttempted,

    #[error("Attempted to delete a Generator")]
    DeleteAttempted,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
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

impl From<WasmError> for Error {
    fn from(e: WasmError) -> Error {
        Error::Wasm(e)
    }
}
