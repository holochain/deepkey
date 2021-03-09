use hdk::prelude::*;
use crate::change_rule::error::Error;
use crate::change_rule::entry::ChangeRule;
use crate::keyset_root::entry::KeysetRoot;

#[hdk_extern]
fn validate_create_entry_key_change_rule(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let element = validate_data.element;
    let (header_hashed, entry) = element.into_inner();
    let (signed_header, _) = header_hashed.into_inner();

    let key_change_rule = match entry {
        ElementEntry::Present(serialized_key_change_rule) => {
            match ChangeRule::try_from(serialized_key_change_rule) {
                Ok(key_change_rule) => key_change_rule,
                Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string())),
            }
        }
        _ => return Ok(ValidateCallbackResult::Invalid(Error::EntryMissing.to_string())),
    };

    // The KSR needs to reference the author as the FDA.
    let keyset_root_element = match get(key_change_rule.keyset_root.clone(), GetOptions::content())? {
        Some(keyset_root) => keyset_root,
        None => return Ok(ValidateCallbackResult::UnresolvedDependencies(vec![key_change_rule.keyset_root.into()]))
    };

    let (keyset_root_header_hashed, keyset_root_entry) = keyset_root_element.into_inner();
    let (_keyset_root_signed_header, _) = keyset_root_header_hashed.into_inner();

    let keyset_root = match keyset_root_entry {
        ElementEntry::Present(serialized_keyset_root) => match KeysetRoot::try_from(serialized_keyset_root) {
            Ok(keyset_root) => keyset_root,
            Err(e) => return Ok(ValidateCallbackResult::Invalid(Error::Wasm(e).to_string())),
        },
        _ => return Ok(ValidateCallbackResult::Invalid(Error::KeysetRootEntryMissing.to_string())),
    };

    if keyset_root.as_first_deepkey_agent_ref() != signed_header.header().author() {
        return Ok(
            ValidateCallbackResult::Invalid(
                Error::AuthorNotFda.to_string()
            )
        );
    }

    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_update_entry_key_change_rule(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::UpdateAttempted.to_string()))
}

#[hdk_extern]
fn validate_delete_entry_key_change_rule(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::DeleteAttempted.to_string()))
}