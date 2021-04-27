use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong KeyRegistration op for this KeyAnchor")]
    RegistrationWrongOp,

    #[error("Wrong key on KeyRegistration for this KeyAnchor")]
    RegistrationWrongKey,

    #[error("Wrong header on KeyRegistration for this KeyAnchor")]
    RegistrationWrongHeader,

    #[error("No KeyRegistration prior to KeyAnchor")]
    RegistrationNone,
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        ValidateCallbackResult::Invalid(e.to_string())
    }
}

impl From<Error> for WasmError {
    fn from(e: Error) -> Self {
        WasmError::Guest(e.to_string())
    }
}

impl From<Error> for ExternResult<ValidateCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}

impl From<Error> for ExternResult<HeaderHash> {
    fn from(e: Error) -> Self {
        Err(e.into())
    }
}