use hdk::prelude::*;
use crate::key_registration::entry::KeyRegistration;
use crate::validate::resolve_dependency;
use crate::validate::ResolvedDependency;
use crate::generator::entry::Generator;
use crate::key_registration::entry::KeyRevocation;
use crate::key_registration::entry::KeyGeneration;
use crate::change_rule::entry::ChangeRule;
use crate::key_registration::error::Error;

fn _validate_key_generation(validate_data: &ValidateData, key_generation: &KeyGeneration) -> ExternResult<ValidateCallbackResult> {
    let (generator_element, generator) = match resolve_dependency::<Generator>(key_generation.as_generator_ref().to_owned().into())? {
        Ok(ResolvedDependency(generator_element, generator)) => (generator_element, generator),
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    if generator_element.header().author() != validate_data.element.header().author() {
        return Error::BadAuthor.into()
    }

    if verify_signature_raw(
        generator.as_change_ref().as_new_key_ref().to_owned(),
        key_generation.as_generator_signature_ref().to_owned(),
        key_generation.as_new_key_ref().as_ref().to_vec()
    )? {
        Ok(ValidateCallbackResult::Valid)
    }
    else {
        Error::BadGeneratorSignature.into()
    }
}

fn _validate_key_revocation(validate_data: &ValidateData, prior_key_change_rule: &ChangeRule, key_revocation: &KeyRevocation) -> ExternResult<ValidateCallbackResult> {
    match validate_data.element.header() {
        Header::Update(update) => if &update.original_header_address != key_revocation.as_prior_key_registration_ref() {
            return Error::BadHeaderRef.into()
        },
        Header::Delete(delete) => if &delete.deletes_address != key_revocation.as_prior_key_registration_ref() {
            return Error::BadHeaderRef.into()
        },
        _ => return Error::BadOp.into(),
    }

    match prior_key_change_rule.authorize(
        key_revocation.as_revocation_authorization_ref(),
        key_revocation.as_prior_key_registration_ref().get_raw_32()
    ) {
        Ok(_) => Ok(ValidateCallbackResult::Valid),
        Err(e) => e.into(),
    }
}

#[hdk_extern]
fn validate_create_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_registration = match KeyRegistration::try_from(&validate_data.element) {
        Ok(key_registration) => key_registration,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match proposed_key_registration {
        KeyRegistration::Create(key_generation) => _validate_key_generation(&validate_data, &key_generation),
        _ => Error::BadOp.into(),
    }
}

#[hdk_extern]
fn validate_update_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_registration = match KeyRegistration::try_from(&validate_data.element) {
        Ok(key_registration) => key_registration,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match proposed_key_registration {
        KeyRegistration::Update(proposed_key_revocation, proposed_key_generation) => {
            match _validate_key_generation(&validate_data, &proposed_key_generation)? {
                ValidateCallbackResult::Valid => { },
                valdiate_callback_result => return Ok(valdiate_callback_result),
            }

            let prior_key_registration: KeyRegistration = match resolve_dependency(proposed_key_revocation.as_prior_key_registration_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, prior_key_registration)) => prior_key_registration,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };

            match prior_key_registration {
                KeyRegistration::Create(prior_key_generation) | KeyRegistration::Update(_, prior_key_generation) => {
                    let prior_generator: Generator = match resolve_dependency(prior_key_generation.as_generator_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_generator)) => prior_generator,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    let prior_key_change_rule: ChangeRule = match resolve_dependency(prior_generator.as_change_rule_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_change_rule)) => prior_change_rule,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    _validate_key_revocation(&validate_data, &prior_key_change_rule, &proposed_key_revocation)
                },
                _ => Error::RevokeRevoke.into(),
            }
        },
        _ => Error::BadOp.into(),
    }
}

#[hdk_extern]
fn validate_delete_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_registration = match KeyRegistration::try_from(&validate_data.element) {
        Ok(key_registration) => key_registration,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match proposed_key_registration {
        KeyRegistration::Delete(proposed_key_revocation) => {
            let prior_key_registration: KeyRegistration = match resolve_dependency(proposed_key_revocation.as_prior_key_registration_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, prior_key_registration)) => prior_key_registration,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };

            match prior_key_registration {
                KeyRegistration::Create(prior_key_generation) | KeyRegistration::Update(_, prior_key_generation) => {
                    let prior_generator: Generator = match resolve_dependency(prior_key_generation.as_generator_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_generator)) => prior_generator,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    let prior_key_change_rule: ChangeRule = match resolve_dependency(prior_generator.as_change_rule_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_change_rule)) => prior_change_rule,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    _validate_key_revocation(&validate_data, &prior_key_change_rule, &proposed_key_revocation)
                },
                _ => Error::RevokeRevoke.into(),
            }
        },
        _ => Error::BadOp.into()
    }
}