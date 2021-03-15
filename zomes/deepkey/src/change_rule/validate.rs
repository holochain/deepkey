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

fn _validate_keyset_root(validate_data: &ValidateData, _: &ChangeRule, keyset_root: &KeysetRoot) -> ExternResult<ValidateCallbackResult> {
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
    // Signed by throwaway RootKey on Create, must have exactly one signature.
    if change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().len() > 1 {
        return Ok(ValidateCallbackResult::Invalid(Error::MultipleCreateSignatures.to_string()));
    }
    let authorization_signature = match change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().iter().next() {
        Some(signature) => signature,
        None => return Ok(ValidateCallbackResult::Invalid(Error::NoCreateSignature.to_string())),
    };

    // The signature must be valid.
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

    let keyset_root: KeysetRoot = match resolve_dependency(change_rule.as_keyset_root_ref().clone().into())? {
        Ok(ResolvedDependency(_, keyset_root)) => keyset_root,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    match _validate_keyset_root(&validate_data, &change_rule, &keyset_root)? {
        ValidateCallbackResult::Valid => { },
        validate_callback_result => return Ok(validate_callback_result),
    }

    match _validate_create_spec_change(&validate_data, &change_rule, &keyset_root)? {
        ValidateCallbackResult::Valid => { },
        validate_callback_result => return Ok(validate_callback_result),
    }

    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_update_entry_key_change_rule(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    // Ok(ValidateCallbackResult::Invalid(Error::UpdateAttempted.to_string()))
    // or according to previous AuthSpec upon Update.
    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
fn validate_delete_entry_key_change_rule(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Invalid(Error::DeleteAttempted.to_string()))
}

#[cfg(test)]
pub mod tests {
    use ::fixt::prelude::*;
    use holochain_types::prelude::*;
    use crate::change_rule::entry::ChangeRuleFixturator;
    use crate::keyset_root::entry::KeysetRootFixturator;
    use crate::change_rule::error::Error;

    #[test]
    fn test_validate_delete() {
        assert_eq!(
            Ok(ValidateCallbackResult::Invalid(Error::DeleteAttempted.to_string())),
            super::validate_delete_entry_key_change_rule(fixt!(ValidateData)),
        );
    }

    #[test]
    fn test_validate_create() {
        // Random garbage won't have a valid ChangeRule on it.
        assert_eq!(
            Ok(ValidateCallbackResult::Invalid("Element missing its ChangeRule".to_string())),
            super::validate_create_entry_key_change_rule(fixt!(ValidateData)),
        );

        let mut validate_data = fixt!(ValidateData);
        let change_rule = fixt!(ChangeRule);

        validate_data.element.entry = ElementEntry::Present(change_rule.clone().try_into().unwrap());

        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_root_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(None));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            Ok(
                ValidateCallbackResult::UnresolvedDependencies(vec![change_rule.as_keyset_root_ref().clone().into()])
            ),
            super::validate_create_entry_key_change_rule(validate_data),
        );
    }

    #[test]
    fn test_validate_create_keyset_root() {
        let mut validate_data = fixt!(ValidateData);
        let change_rule = fixt!(ChangeRule);
        let keyset_root = fixt!(KeysetRoot);
        let mut create_header = fixt!(Create);

        validate_data.element.signed_header.header.content = Header::Create(create_header.clone());

        // The FDA cannot be valid unless the validation element and keyset root FDA are the same.
        assert_eq!(
            Ok(
                ValidateCallbackResult::Invalid(
                    Error::AuthorNotFda.to_string()
                )
            ),
            super::_validate_keyset_root(&validate_data, &change_rule, &keyset_root),
        );

        create_header.author = keyset_root.as_first_deepkey_agent_ref().clone();
        validate_data.element.signed_header.header.content = Header::Create(create_header);

        assert_eq!(
            Ok(ValidateCallbackResult::Valid),
            super::_validate_keyset_root(&validate_data, &change_rule, &keyset_root),
        );
    }

    #[test]
    fn test_validate_create_spec_change() {
        let validate_data = fixt!(ValidateData);
        let mut change_rule = fixt!(ChangeRule);
        let keyset_root = fixt!(KeysetRoot);

        change_rule.spec_change.authorization_of_new_spec.push(fixt!(Signature));
        change_rule.spec_change.authorization_of_new_spec.push(fixt!(Signature));

        // Too many sigs fails.
        assert_eq!(
            Ok(ValidateCallbackResult::Invalid(
                Error::MultipleCreateSignatures.to_string()
            )),
            super::_validate_create_spec_change(&validate_data, &change_rule, &keyset_root),
        );

        change_rule.spec_change.authorization_of_new_spec = vec![];

        // No sig fails.
        assert_eq!(
            Ok(ValidateCallbackResult::Invalid(
                Error::NoCreateSignature.to_string()
            )),
            super::_validate_create_spec_change(&validate_data, &change_rule, &keyset_root),
        );

        // Invalid sig fails.
        let signature = fixt!(Signature);
        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(VerifySignature::new(
                keyset_root.as_root_pub_key_ref().clone(),
                signature.clone(),
                change_rule.as_spec_change_ref().as_new_spec_ref().clone()
            ).unwrap()))
            .times(1)
            .return_const(Ok(false));

        hdk::prelude::set_hdk(mock_hdk);

        change_rule.spec_change.authorization_of_new_spec = vec![signature];

        assert_eq!(
            Ok(ValidateCallbackResult::Invalid(
                Error::BadCreateSignature.to_string()
            )),
            super::_validate_create_spec_change(&validate_data, &change_rule, &keyset_root),
        );

        // Valid sig passes.
        let mut mock_hdk = hdk::prelude::MockHdkT::new();
        mock_hdk.expect_verify_signature()
            .times(1)
            .return_const(Ok(true));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            Ok(ValidateCallbackResult::Valid),
            super::_validate_create_spec_change(&validate_data, &change_rule, &keyset_root),
        );
    }
}