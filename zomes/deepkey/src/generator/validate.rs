use hdk::prelude::*;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::change_rule::entry::ChangeRule;
use crate::generator::entry::Generator;

#[hdk_extern]
fn validate_create_entry_generator(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_generator = match Generator::try_from(&validate_data.element) {
        Ok(generator) => generator,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    let change_rule: ChangeRule = match resolve_dependency(proposed_generator.as_change_rule_ref().clone().into())? {
        Ok(ResolvedDependency(_, change_rule)) => change_rule,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    match change_rule.authorize(
        proposed_generator.as_change_ref().as_authorization_ref().to_vec(),
        holochain_serialized_bytes::encode(&proposed_generator.as_change_ref().as_new_key_ref())?,
    ) {
        Ok(_) => Ok(ValidateCallbackResult::Valid),
        Err(e) => e.into(),
    }
}