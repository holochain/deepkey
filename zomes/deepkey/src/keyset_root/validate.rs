use hdk::prelude::*;
use crate::keyset_root::entry;

impl entry::KeysetRoot {
    pub fn signature_is_valid(&self) -> ExternResult<bool> {
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
        // Header needs to be a creat.
        Header::Create(create_header) => {
            // Header needs to be in the correct position in the chain.
            if create_header.header_seq == entry::KEYSET_ROOT_CHAIN_INDEX {
                match entry {
                    // Element needs to be present for creation.
                    ElementEntry::Present(serialized_keyset_root) => {
                        // KeysetRoot needs to deserialize cleanly from entry.
                        match entry::KeysetRoot::try_from(serialized_keyset_root) {
                            Ok(keyset_root) => {
                                // Signature needs to be right.
                                if keyset_root.signature_is_valid()? {
                                    // Source = FirstDeepkeyAgent
                                    // When new KeysetRoot is created -> validate that Author=FirstDeepKeyAgent
                                    if &create_header.author == keyset_root.as_first_deepkey_agent_ref() {
                                        Ok(ValidateCallbackResult::Valid)
                                    }
                                    else {
                                        Ok(ValidateCallbackResult::Invalid("KeysetRoot create author must be the FDA.".to_string()))
                                    }
                                }
                                else {
                                    Ok(ValidateCallbackResult::Invalid("Invalid signature from root key for FDA key.".to_string()))
                                }
                            },
                            Err(e) => Ok(ValidateCallbackResult::Invalid(e.to_string())),
                        }
                    },
                    _ => Ok(ValidateCallbackResult::Invalid("No present KeysetRoot entry.".to_string()))
                }
            }
            else {
                Ok(ValidateCallbackResult::Invalid(format!("KeysetRoot must have chain index {}", entry::KEYSET_ROOT_CHAIN_INDEX)))
            }
        },
        _ => Ok(ValidateCallbackResult::Invalid("Not a create header for new KeysetRoot entry creation.".to_string())),
    }
}

#[hdk_extern]
/// Updates are not allowed for KeysetRoot.
fn validate_update_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("KeysetRoot update is not allowed.".to_string()))
}

#[hdk_extern]
/// Deletes are not allowed for KeysetRoot.
fn validate_delete_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("KeysetRoot delete is not allowed.".to_string()))
}