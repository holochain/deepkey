use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element missing its KeysetRoot")]
    EntryMissing,

    #[error("Attempted to delete a KeysetRoot")]
    DeleteAttempted,

    #[error("Attempted to update a KeysetRoot")]
    UpdateAttempted,

    #[error("Attempted to create KeysetRoot in position {0} expected {1}")]
    Position(u32, u32),

    #[error("Bad FDA signature in KeysetRoot")]
    FdaSignature,

    #[error("Element author does not match FDA for KeysetRoot")]
    FdaAuthor,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}