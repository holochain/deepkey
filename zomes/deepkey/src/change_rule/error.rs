use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Element missing its ChangeRule")]
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

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}