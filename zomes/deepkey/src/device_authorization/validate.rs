use hdk::prelude::*;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::entry::DeviceAuthorization;
use crate::device_authorization::error::Error;

#[hdk_extern]
fn validate_create_entry_device_authorization(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validate_data.element;
    let (header_hashed, entry) = element.into_inner();
    let (signed_header, _) = header_hashed.into_inner();

    // Header is a create as per validate callback name.
    let create_header = match signed_header.header() {
        Header::Create(create_header) => create_header,
        // Holochain sent us the wrong header!.
        _ => unreachable!(),
    };

    let device_authorization = match entry {
        ElementEntry::Present(serialized_device_authorization) => {
            match DeviceAuthorization::try_from(serialized_device_authorization) {
                Ok(device_authorization) => device_authorization,
                Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string()))
            }
        },
        _ => return Ok(ValidateCallbackResult::Invalid(Error::EntryMissing.to_string())),
    };

    // Is parent found and valid on the DHT?
    // The parent can be in any CRUD state for the purpose of validation.
    // Both the only valid parent types, KSR and DA are create-only.
    let parent_element = match get(device_authorization.as_parent_ref().to_owned(), GetOptions::content())? {
        Some(parent_element) => parent_element,
        None => return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_authorization.as_parent_ref().to_owned().into()])),
    };

    // Are both acceptors unique, found and valid on the DHT?
    // The acceptors can be in any CRUD state for the purpose of validation.
    if device_authorization.as_device_acceptance_ref().0 == device_authorization.as_root_acceptance_ref().0 {
        return Ok(ValidateCallbackResult::Invalid(Error::UniqueAgent.to_string()));
    }
    if !get(device_authorization.as_root_acceptance_ref().0.clone(), GetOptions::content())?.is_some() {
        return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_authorization.as_root_acceptance_ref().0.clone().into()]));
    }
    if !get(device_authorization.as_device_acceptance_ref().0.clone(), GetOptions::content())?.is_some() {
        return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_authorization.as_device_acceptance_ref().0.clone().into()]));
    }

    // Signatures.
    if !device_authorization.verify_signatures()? {
        return Ok(ValidateCallbackResult::Invalid(Error::Signature.to_string()));
    }

    let (parent_header_hashed, parent_entry) = parent_element.into_inner();
    let (parent_signed_header, _) = parent_header_hashed.into_inner();

    let parent_create_header = match parent_signed_header.header() {
        Header::Create(parent_create_header) => parent_create_header,
        _ => return Ok(ValidateCallbackResult::Invalid(Error::ParentNotCreate.to_string())),
    };

    let serialized_parent_entry = match parent_entry {
        ElementEntry::Present(serialized_parent_entry) => serialized_parent_entry,
        _ => return Ok(ValidateCallbackResult::Invalid(Error::ParentEntryMissing.to_string())),
    };

    // Parent is a KSA.
    if parent_create_header.entry_type == entry_type!(KeysetRoot)? {
        // Clean KSR deserialize.
        let keyset_root = match KeysetRoot::try_from(serialized_parent_entry) {
            Ok(keyset_root) => keyset_root,
            Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string())),
        };

        // The parent _is_ the KeysetRoot.
        // Is it the _right_ KeysetRoot?
        if device_authorization.as_parent_ref() != device_authorization.as_keyset_root_authority_ref() {
            return Ok(ValidateCallbackResult::Invalid(Error::WrongKeysetRoot.to_string()));
        }

        // DA author must be the KSR FDA.
        // KSA's own authorship integrity is ensured by the KSR's validation.
        if keyset_root.as_first_deepkey_agent_ref() != &create_header.author {
            return Ok(ValidateCallbackResult::Invalid(Error::WrongFda.to_string()));
        }

        // DA author must be the root authorizer.
        if device_authorization.as_root_acceptance_ref().0 != create_header.author {
            return Ok(ValidateCallbackResult::Invalid(Error::FdaAsDeviceAcceptance.to_string()));
        }
    }
    // Parent is a DA one hop closer to the KSRA.
    else if parent_create_header.entry_type == entry_type!(DeviceAuthorization)? {
        // Clean DA deserialize.
        let parent_device_authorization = match DeviceAuthorization::try_from(serialized_parent_entry) {
            Ok(parent_device_authorization) => parent_device_authorization,
            Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string())),
        };

        // The parent must _not_ be the KSR.
        if device_authorization.as_parent_ref() == device_authorization.as_keyset_root_authority_ref() {
            return Ok(ValidateCallbackResult::Invalid(Error::DeviceAuthorizationAuthority.to_string()));
        }

        // The author must be _either_ of the acceptors.
        if (device_authorization.as_root_acceptance_ref().0 != create_header.author) && (device_authorization.as_device_acceptance_ref().0 != create_header.author) {
            return Ok(ValidateCallbackResult::Invalid(Error::DeviceAuthorizationAuthor.to_string()));
        }

        // If the author is on the root side it must also be one of the parent acceptors.
        if device_authorization.as_root_acceptance_ref().0 == create_header.author {
            if (parent_device_authorization.as_root_acceptance_ref().0 != create_header.author) && (parent_device_authorization.as_device_acceptance_ref().0 != create_header.author) {
                return Ok(ValidateCallbackResult::Invalid(Error::DeviceAuthorizationParentAuthor.to_string()))
            }
        }

        // The parent must have the same KSR.
        if device_authorization.as_keyset_root_authority_ref() != parent_device_authorization.as_keyset_root_authority_ref() {
            return Ok(ValidateCallbackResult::Invalid(Error::WrongKeysetRoot.to_string()));
        }
    }
    else {
        return Ok(ValidateCallbackResult::Invalid(Error::ParentEntryType.to_string()))
    }

    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
/// Updates are not allowed for DeviceAuthorization.
fn validate_update_entry_device_authorization(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::UpdateAttempted.to_string()))
}

#[hdk_extern]
/// Deletes are not allowed for DeviceAuthorization.
fn validate_delete_entry_device_authorization(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::DeleteAttempted.to_string()))
}