use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone)]
pub struct FirstEntry {
    pub name: String,
    pub description: String,
    pub age: u32,
}
