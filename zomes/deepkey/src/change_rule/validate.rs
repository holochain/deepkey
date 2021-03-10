use hdk::prelude::*;
use crate::change_rule::error::Error;
use crate::change_rule::entry::ChangeRule;
use crate::keyset_root::entry::KeysetRoot;

impl TryFrom<&Element> for ChangeRule {
    type Error = Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        Ok(match element.entry() {
            ElementEntry::Present(serialized_change_rule) => match ChangeRule::try_from(serialized_change_rule) {
                Ok(change_rule) => change_rule,
                Err(e) => return Err(Error::Wasm(e)),
            }
            __ => return Err(Error::EntryMissing),
        })
    }
}

struct ResolvedDependency<D>(pub Element, pub D);

fn resolve_dependency<'a, O>(hash: AnyDhtHash) -> ExternResult<Result<ResolvedDependency<O>, ValidateCallbackResult>>
    where
        O: TryFrom<SerializedBytes, Error = SerializedBytesError>
        {
    let element = match get(hash.clone(), GetOptions::content())? {
        Some(element) => element,
        None => return Ok(Err(ValidateCallbackResult::UnresolvedDependencies(vec![hash]))),
    };

    let output: O = match element.entry().to_app_option() {
        Ok(Some(output)) => output,
        Ok(None) => return Ok(Err(ValidateCallbackResult::Invalid(Error::EntryMissing.to_string()))),
        Err(e) => return Ok(Err(ValidateCallbackResult::Invalid(e.to_string()))),
    };

    Ok(Ok(ResolvedDependency(element, output)))
}

fn _validate_keyset_root(validate_data: &ValidateData, change_rule: &ChangeRule) -> ExternResult<ValidateCallbackResult> {
    let keyset_root: KeysetRoot = match resolve_dependency(change_rule.as_keyset_root_ref().clone().into())? {
        Ok(ResolvedDependency(_, keyset_root)) => keyset_root,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    // // The KSR needs to reference the author as the FDA.
    if keyset_root.as_first_deepkey_agent_ref() != validate_data.element.signed_header().header().author() {
        return Ok(
            ValidateCallbackResult::Invalid(
                Error::AuthorNotFda.to_string()
            )
        );
    }
    Ok(ValidateCallbackResult::Valid)
}

fn _validate_create_spec_change(_: &ValidateData, change_rule: &ChangeRule, keyset_root: &KeysetRoot) -> ExternResult<ValidateCallbackResult> {
    // signed by throwaway RootKey on Create,
    if change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().len() != 1 {
        return Ok(ValidateCallbackResult::Invalid(Error::MultipleCreateSignatures.to_string()));
    }

    let authorization_signature = match change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().iter().next() {
        Some(signature) => signature,
        None => return Ok(ValidateCallbackResult::Invalid(Error::NoCreateSignature.to_string())),
    };

    if verify_signature(
        keyset_root.as_root_pub_key_ref().clone(),
        authorization_signature.clone(),
        change_rule.as_spec_change_ref().as_new_spec_ref()
    )? {
        Ok(ValidateCallbackResult::Valid)
    } else {
        Ok(ValidateCallbackResult::Invalid(Error::BadCreateSignature.to_string()))
    }
}

#[hdk_extern]
fn validate_create_entry_key_change_rule(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let change_rule = match ChangeRule::try_from(&validate_data.element) {
        Ok(key_change_rule) => key_change_rule,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match _validate_keyset_root(&validate_data, &change_rule)? {
        ValidateCallbackResult::Valid => { },
        validate_callback_result => return Ok(validate_callback_result),
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