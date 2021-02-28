use hdk::prelude::*;
use crate::keyset_root::entry;
use crate::keyset_root::error::Error;

impl entry::KeysetRoot {
    pub fn verify_signature(&self) -> ExternResult<bool> {
        verify_signature_raw(
            self.as_root_pub_key_ref().to_owned(),
            self.as_fda_pubkey_signed_by_root_key_ref().to_owned(),
            self.as_first_deepkey_agent_ref().get_raw_32().to_vec()
        )
    }
}

#[hdk_extern]
/// Create only.
fn validate_create_entry_keyset_root(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validate_data.element;
    let (header_hashed, entry) = element.into_inner();
    let (signed_header, _) = header_hashed.into_inner();

    match signed_header.header() {
        // Header needs to be a create.
        Header::Create(create_header) => {
            // Header needs to be in the correct position in the chain.
            if create_header.header_seq == entry::KEYSET_ROOT_CHAIN_INDEX {
                match entry {
                    // Element needs to be present for creation.
                    ElementEntry::Present(serialized_keyset_root) => {
                        // Clean deserialize.
                        match entry::KeysetRoot::try_from(serialized_keyset_root) {
                            Ok(keyset_root) => {
                                // Valid signature.
                                if keyset_root.verify_signature()? {
                                    // Source = FirstDeepkeyAgent
                                    // When new KeysetRoot is created -> validate that Author=FirstDeepKeyAgent
                                    if keyset_root.as_first_deepkey_agent_ref() == &create_header.author {
                                        Ok(ValidateCallbackResult::Valid)
                                    }
                                    else {
                                        Ok(ValidateCallbackResult::Invalid(Error::FdaAuthor.to_string()))
                                    }
                                }
                                else {
                                    Ok(ValidateCallbackResult::Invalid(Error::FdaSignature.to_string()))
                                }
                            },
                            Err(e) => Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string()))
                        }
                    },
                    _ => Ok(ValidateCallbackResult::Invalid(Error::EntryMissing.to_string()))
                }
            }
            else {
                Ok(ValidateCallbackResult::Invalid(Error::Position(create_header.header_seq, entry::KEYSET_ROOT_CHAIN_INDEX).to_string()))
            }
        },
        // Holochain sent a non-Create Header to `validate_create_`!
        _ => unreachable!(),
    }
}

#[hdk_extern]
/// Updates are not allowed for KeysetRoot.
fn validate_update_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::UpdateAttempted.to_string()))
}

#[hdk_extern]
/// Deletes are not allowed for KeysetRoot.
fn validate_delete_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::DeleteAttempted.to_string()))
}