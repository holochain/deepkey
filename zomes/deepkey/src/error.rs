use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element missing its Entry")]
    EntryMissing,

    #[error("Wrong header for an Element")]
    WrongHeader,

    #[error("Keyset sequence is wrong")]
    KeysetSeq,

    #[error("KeyRegistration sequence is wrong")]
    KeyRegistrationSeq,

    #[error("KeysetRoot not followed by ChangeRule")]
    KeysetRootThenChangeRule,

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