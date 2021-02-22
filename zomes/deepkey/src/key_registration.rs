use hdk::prelude::*;
use crate::key;

struct CreateKeyRegistration {
    key: key::Key,
    authorization_sig: Signature,
}

struct UpdateKeyRegistration {
    key: key::Key,
    authorization_sig: Signature,
    prior_key: key::Key,
    revocation_sig: Signature,
}

#[hdk_entry(id = "key_registration")]
enum KeyRegistration {
    Create(CreateKeyRegistration),
    Update(UpdateKeyRegistration),
}

#[hdk_extern]
fn create_key_registration(create: CreateKeyRegistration) -> ExternResult<HeaderHash> {
    create_entry(create)
}

#[hdk_extern]
fn update_key_registration(update: UpdateKeyRegistration) -> ExternResult<HeaderHash> {
    create_entry(update)
}

#[hdk_extern]
fn delete_key_registration(old_key_registration: HeaderHash) -> ExternResult<HeaderHash> {
    delete_entry(old_key_registration)
}