use hdk::prelude::*;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::change_rule::entry::ChangeRule;
use crate::generator::entry::Generator;
use crate::generator::error::Error;

fn _validate_create_entry_generator_authorize(change_rule: &ChangeRule, generator: &Generator) -> ExternResult<ValidateCallbackResult> {
    match change_rule.authorize(
        generator.as_change_ref().as_authorization_ref(),
        &holochain_serialized_bytes::encode(&generator.as_change_ref().as_new_key_ref())?,
    ) {
        Ok(_) => Ok(ValidateCallbackResult::Valid),
        Err(e) => e.into(),
    }
}

#[hdk_extern]
fn validate_create_entry_generator(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_generator = match Generator::try_from(&validate_data.element) {
        Ok(generator) => generator,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    let (_, change_rule) = match resolve_dependency::<ChangeRule>(proposed_generator.as_change_rule_ref().clone().into())? {
        Ok(ResolvedDependency(change_rule_element, change_rule)) => (change_rule_element, change_rule),
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    _validate_create_entry_generator_authorize(&change_rule, &proposed_generator)
}

#[hdk_extern]
fn validate_update_entry_generator(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
fn validate_delete_entry_generator(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}

#[cfg(test)]
pub mod test {
    use hdk::prelude::*;
    use holochain_types::prelude::*;
    use ::fixt::prelude::*;
    use crate::generator::error::Error;
    use crate::generator::entry::GeneratorFixturator;

    #[test]
    fn test_validate_create_entry_generator_authorize() {
        // See tests for ChangeRule::authorize()
    }

    #[test]
    fn test_validate_create_entry_generator() {
        let mut validate_data = fixt!(ValidateData);
        let generator = fixt!(Generator);
        let create_header = fixt!(Create);
        *validate_data.element.as_header_mut() = Header::Create(create_header.clone());

        assert_eq!(
            super::validate_create_entry_generator(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        *validate_data.element.as_entry_mut() = ElementEntry::Present(generator.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
        .with(mockall::predicate::eq(
            GetInput::new(
                generator.as_change_rule_ref().clone().into(),
                GetOptions::content()
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            Ok(
                ValidateCallbackResult::UnresolvedDependencies(vec![generator.as_change_rule_ref().clone().into()])
            ),
            super::validate_create_entry_generator(validate_data.clone())
        );
    }

    #[test]
    fn test_validate_update_entry_generator() {
        assert_eq!(
            super::validate_update_entry_generator(fixt!(ValidateData)),
            Error::UpdateAttempted.into(),
        );
    }

    #[test]
    fn test_validate_delete_entry_generator() {
        assert_eq!(
            super::validate_delete_entry_generator(fixt!(ValidateData)),
            Error::DeleteAttempted.into(),
        );
    }
}