use hdk::prelude::*;
use crate::device_authorization::entry;

#[hdk_extern]
fn create_device_authorization(device: entry::DeviceAuthorization) -> ExternResult<HeaderHash> {
    create_entry(device)
}