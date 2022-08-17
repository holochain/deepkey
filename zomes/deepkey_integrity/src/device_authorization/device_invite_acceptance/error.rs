use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Wrong header for a DeviceInviteAcceptance Record")]
    WrongHeader,

    #[error("Wrong author for a DeviceInviteAcceptance as per its DeviceInvite")]
    WrongAuthor,

    #[error("Wrong KeysetRoot for a DeviceInviteAcceptance as per its DeviceInvite")]
    WrongKeysetRoot,

    #[error("Attempted to update a DeviceInviteAcceptance")]
    UpdateAttempted,

    #[error("Attempted to delete a DeviceInviteAcceptance")]
    DeleteAttempted,

    #[error("Record is missing DeviceInviteAcceptance entry")]
    EntryMissing,

    #[error("Cannot find invite to accept")]
    InviteNotFound,

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
