pub mod first_entry;
pub use first_entry::*;
use hdi::prelude::*;
#[hdk_extern]
pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    FirstEntry(FirstEntry),
}
#[hdk_link_types]
pub enum LinkTypes {
    FirstEntryUpdates,
}
