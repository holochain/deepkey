use hdk::prelude::*;

/// KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.
const KEYSET_ROOT_CHAIN_INDEX: u32 = 3;

#[hdk_entry(id = "keyset_root")]
/// We need an entry to create a permanent anchor that can be used to reference the space of keys under the control of a human agent.
/// This is commited only by the FirstDeepkeyAgent (FDA) not later devices that are joining this same agency context.
pub struct KeysetRoot {
    first_deepkey_agent: AgentPubKey,
    /// The private key is thrown away.
    root_pub_key: AgentPubKey,
    fda_pubkey_signed_by_root_key: Signature,
}

impl KeysetRoot {
    pub fn signature_is_valid(&self) -> ExternResult<bool> {
        verify_signature_raw(
            self.root_pub_key.clone(),
            self.fda_pubkey_signed_by_root_key.clone(),
            self.first_deepkey_agent.get_raw_32().to_vec()
        )
    }
}

#[hdk_extern]
fn create_keyset_root(new_keyset_root: KeysetRoot) -> ExternResult<HeaderHash> {
    create_entry(new_keyset_root)
}

#[hdk_extern]
/// Create only.
fn validate_create_entry_keyset_root(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validate_data.element;
    let (header_hashed, entry) = element.into_inner();
    let (signed_header, _) = header_hashed.into_inner();

    match signed_header.header() {
        Header::Create(create_header) => {
            if create_header.header_seq == KEYSET_ROOT_CHAIN_INDEX {
                match entry {
                    ElementEntry::Present(serialized_keyset_root) => {
                        match KeysetRoot::try_from(serialized_keyset_root) {
                            Ok(keyset_root) => {
                                if keyset_root.signature_is_valid()? {
                                    Ok(ValidateCallbackResult::Valid)
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
                Ok(ValidateCallbackResult::Invalid(format!("KeysetRoot must have chain index {}", KEYSET_ROOT_CHAIN_INDEX)))
            }
        },
        _ => Ok(ValidateCallbackResult::Invalid("Not a create header for new KeysetRoot entry creation.".to_string())),
    }
}

#[hdk_extern]
fn validate_update_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("KeysetRoot update is not allowed.".to_string()))
}

#[hdk_extern]
fn validate_delete_entry_keyset_root(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid("KeysetRoot delete is not allowed.".to_string()))
}