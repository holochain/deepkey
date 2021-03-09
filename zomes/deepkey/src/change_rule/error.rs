use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element missing its DeviceAuthorization")]
    EntryMissing,

    #[error("Attempted to delete a DeviceAuthorization")]
    DeleteAttempted,

    #[error("Attempted to update a DeviceAuthorization")]
    UpdateAttempted,

    #[error("KeysetRoot entry is missing from Element")]
    KeysetRootEntryMissing,

    #[error("The KeyChangeRule author is not the FDA on the KeysetRoot")]
    AuthorNotFda,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}