use hdk::prelude::*;
use thiserror::Error;

use crate::keyset_root::entry::KeysetRoot;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing Device")]
    MissingDevice,

    #[error("Not a DeepKeyAgent Entry")]
    EntryNotDeepKeyAgent,

    #[error("Attempted to delete a DeviceAuthorization")]
    DeviceAuthorizationDelete,

    #[error("Attempted to update a DeviceAuthorization")]
    DeviceAuthorizationUpdate,
}