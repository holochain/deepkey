use hdk::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Not an agent link tag")]
    NotAgentTag,
}

impl From<Error> for WasmError {
    fn from(e: Error) -> Self {
        Self::Guest(e.to_string())
    }
}