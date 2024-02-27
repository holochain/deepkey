use crate::*;
use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub enum SourceOfAuthority {
    KeysetRoot(KeysetRoot),
}
