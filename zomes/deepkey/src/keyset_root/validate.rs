use hdk::prelude::*;
use crate::keyset_root::entry::KeysetRoot;
use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
use crate::keyset_root::error::Error;

impl KeysetRoot {
    pub fn verify_signature(&self) -> ExternResult<bool> {
        verify_signature_raw(
            self.as_root_pub_key_ref().to_owned(),
            self.as_fda_pubkey_signed_by_root_key_ref().to_owned(),
            self.as_first_deepkey_agent_ref().get_raw_32().to_vec()
        )
    }
}

impl TryFrom<&Element> for KeysetRoot {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        Ok(match element.entry() {
            ElementEntry::Present(serialized_keyset_root) => match KeysetRoot::try_from(serialized_keyset_root) {
                Ok(keyset_root) => keyset_root,
                Err(e) => return Err(Error::Wasm(e)),
            },
            _ => return Err(Error::EntryMissing),
        })
    }
}

#[hdk_extern]
/// Create only.
fn validate_create_entry_keyset_root(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validate_data.element;
    let (header_hashed, entry) = element.into_inner();
    let (signed_header, _) = header_hashed.into_inner();

    // Header is a create as per the validate callback name.
    let create_header = match signed_header.header() {
        Header::Create(create_header) => create_header,
        // Holochain sent the wrong header!
        _ => unreachable!(),
    };

    // Header needs to be in the correct position in the chain.
    if create_header.header_seq != KEYSET_ROOT_CHAIN_INDEX {
        return Ok(ValidateCallbackResult::Invalid(Error::Position(create_header.header_seq, KEYSET_ROOT_CHAIN_INDEX).to_string()));
    }

    let keyset_root = match entry {
        ElementEntry::Present(serialized_keyset_root) => {
            match KeysetRoot::try_from(serialized_keyset_root) {
                Ok(keyset_root) => keyset_root,
                Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string()))
            }
        },
        _ => return Ok(ValidateCallbackResult::Invalid(Error::EntryMissing.to_string()))
    };

    if !keyset_root.verify_signature()? {
        return Ok(ValidateCallbackResult::Invalid(Error::FdaSignature.to_string()));
    }

    if !(*keyset_root.as_first_deepkey_agent_ref() == create_header.author) {
        return Ok(ValidateCallbackResult::Invalid(Error::FdaAuthor.to_string()))
    }

    Ok(ValidateCallbackResult::Valid)
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

#[cfg(test)]
pub mod test {
    use hdk::prelude::*;
    use ::fixt::prelude::*;
    use crate::keyset_root::entry::KeysetRoot;
    use holochain_types::prelude::*;
    use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;

    #[test]
    fn validate_valid_ksr() {
        let mut header_hashes = HeaderHashFixturator::new(Unpredictable);
        let mut agents = AgentPubKeyFixturator::new(Predictable);
        let mut signatures = SignatureFixturator::new(Predictable);
        let mut zome_infos = ZomeInfoFixturator::new(Predictable);

        let root_pub_key = agents.next().unwrap();
        let first_deepkey_agent = agents.next().unwrap();
        let fda_pubkey_signed_by_root_key = signatures.next().unwrap();

        let keyset_root = KeysetRoot::new(
            root_pub_key,
            first_deepkey_agent.clone(),
            fda_pubkey_signed_by_root_key,
        );

        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        let zome_info = zome_infos.next().unwrap();
        mock_hdk.expect_zome_info()
            .with(mockall::predicate::eq(()))
            .times(1)
            .return_once(move |_| Ok(zome_info));

        let _mock_lock = hdk::prelude::set_global_hdk(mock_hdk).unwrap();

        let (_entry, entry_hash) = EntryHashed::from_content_sync(
            Entry::try_from(&keyset_root).unwrap()
        ).into_inner();
        let entry_type = entry_type!(KeysetRoot).unwrap();

        dbg!(&entry_type);

        let header_builder = builder::Create {
            entry_type,
            entry_hash,
        };
        let prev_header = header_hashes.next().unwrap();
        let common = HeaderBuilderCommon {
            author: first_deepkey_agent,
            timestamp: holochain_types::timestamp::now(),
            header_seq: KEYSET_ROOT_CHAIN_INDEX,
            prev_header,
        };
        let header = header_builder.build(common);

        dbg!(&header);
    }
}