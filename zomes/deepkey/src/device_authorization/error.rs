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

    #[error("DeviceAuthorization with identical agents")]
    UniqueAgent,

    #[error("DeviceAuthorization with bad signature")]
    Signature,

    #[error("Parent element is not creation of an entry")]
    ParentNotCreate,

    #[error("Parent element missing an entry")]
    ParentEntryMissing,

    #[error("DeviceAuthorization has the wrong KeysetRoot")]
    WrongKeysetRoot,

    #[error("DeviceAuthorization root author is not the KeysetRoot FDA")]
    WrongFda,

    #[error("DeviceAuthorization root author has FDA is not the root acceptor")]
    FdaAsDeviceAcceptance,

    #[error("DeviceAuthorization parent is the KeysetRoot hash but a DeviceAuthorization entry")]
    DeviceAuthorizationAuthority,

    #[error("DeviceAuthorization author is neither of the acceptors")]
    DeviceAuthorizationAuthor,

    #[error("DeviceAuthorization author is a root acceptor but not referenced by the parent")]
    DeviceAuthorizationParentAcceptor,

    #[error("DeviceAuthorization author is a root acceptor but the parent author is different")]
    DeviceAuthorizationParentAuthor,

    #[error("Parent entry is of the wrong type")]
    ParentEntryType,

    #[error("Wasm error {0}")]
    Wasm(WasmError)
}