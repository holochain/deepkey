use hdk::prelude::*;
use crate::key_registration::entry::KeyRegistration;

#[hdk_extern]
fn create_key_registration(key_registration: KeyRegistration) -> ExternResult<HeaderHash> {
    create_entry(key_registration)
}