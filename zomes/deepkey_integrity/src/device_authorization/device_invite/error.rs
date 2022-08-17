use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong header for a DeviceInvite Record")]
    WrongHeader,

    #[error("Wrong KeysetRoot for a DeviceInvite")]
    WrongKeysetRoot,

    #[error("Wrong author for a DeviceInvite as per its parent")]
    WrongAuthor,

    #[error("Attempted to update a DeviceInvite")]
    UpdateAttempted,

    #[error("Attempted to delete a DeviceInvite")]
    DeleteAttempted,

    #[error("Record is missing DeviceInvite entry")]
    EntryMissing,

    #[error("DeviceInvite author is not the FDA of the parent KeysetRoot")]
    AuthorNotFda,

    #[error("DeviceInvite attempted to self-invite author")]
    SelfInvite,

    #[error("The new DeviceInvite references a stale keyset leaf")]
    StaleKeysetLeaf,

    #[error("The new ChangeRule has no validation package")]
    MissingValidationPackage,

    #[error("No KeysetFound on chain")]
    MissingKeyset,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}

impl From<Error> for ValidateCallbackResult {
    fn from(e: Error) -> Self {
        Self::Invalid(e.to_string())
    }
}

impl From<Error> for WasmError {
    fn from(e: Error) -> Self {
        Self::Guest(e.to_string())
    }
}

impl From<Error> for ExternResult<ValidateCallbackResult> {
    fn from(e: Error) -> Self {
        Ok(e.into())
    }
}
