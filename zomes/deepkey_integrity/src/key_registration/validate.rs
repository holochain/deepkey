use hdk::prelude::*;
use crate::key_registration::entry::KeyRegistration;
use crate::validate::resolve_dependency;
use crate::validate::ResolvedDependency;
use crate::generator::entry::Generator;
use crate::key_registration::entry::KeyRevocation;
use crate::key_registration::entry::KeyGeneration;
use crate::change_rule::entry::ChangeRule;
use crate::key_registration::error::Error;
use crate::validate_classic::*;

fn _validate_key_self_signing(validate_data: &ValidateData, key_generation: &KeyGeneration) -> ExternResult<ValidateCallbackResult> {
    if verify_signature_raw(
        key_generation.as_new_key_ref().to_owned(),
        key_generation.as_new_key_signing_of_author_ref().to_owned(),
        validate_data.element.action().author().get_raw_32().to_vec()
    )? {
        Ok(ValidateCallbackResult::Valid)
    }
    else {
        Error::BadSelfSignature.into()
    }
}

fn _validate_key_generation(validate_data: &ValidateData, key_generation: &KeyGeneration) -> ExternResult<ValidateCallbackResult> {
    let (generator_element, generator) = match resolve_dependency::<Generator>(key_generation.as_generator_ref().to_owned().into())? {
        Ok(ResolvedDependency(generator_element, generator)) => (generator_element, generator),
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    if generator_element.action().author() != validate_data.element.action().author() {
        return Error::BadAuthor.into()
    }

    match _validate_key_self_signing(validate_data, key_generation) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
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
    match validate_data.element.action() {
        Action::Update(update) => if &update.original_action_address != key_revocation.as_prior_key_registration_ref() {
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
        KeyRegistration::Create(key_generation) | KeyRegistration::CreateOnly(key_generation) => _validate_key_generation(&validate_data, &key_generation),
        _ => Error::BadOp.into(),
    }
}

#[hdk_extern]
fn validate_update_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_key_registration = match KeyRegistration::try_from(&validate_data.element) {
        Ok(key_registration) => key_registration,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    // All updates MUST revoke a prior generation.
    match proposed_key_registration {
        KeyRegistration::Update(ref proposed_key_revocation, _) | KeyRegistration::Delete(ref proposed_key_revocation) => {
            let prior_key_registration: KeyRegistration = match resolve_dependency(proposed_key_revocation.as_prior_key_registration_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, prior_key_registration)) => prior_key_registration,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };

            match prior_key_registration {
                KeyRegistration::CreateOnly(_) => return Error::CreateOnlyUpdate.into(),
                KeyRegistration::Create(prior_key_generation) | KeyRegistration::Update(_, prior_key_generation) => {
                    let prior_generator: Generator = match resolve_dependency(prior_key_generation.as_generator_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_generator)) => prior_generator,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    let prior_key_change_rule: ChangeRule = match resolve_dependency(prior_generator.as_change_rule_ref().to_owned().into())? {
                        Ok(ResolvedDependency(_, prior_change_rule)) => prior_change_rule,
                        Err(validate_callback_result) => return Ok(validate_callback_result),
                    };
                    match _validate_key_revocation(&validate_data, &prior_key_change_rule, &proposed_key_revocation) {
                        Ok(ValidateCallbackResult::Valid) => { },
                        validate_callback_result => return validate_callback_result,
                    }
                },
                _ => return Error::Tombstone.into(),
            }
        },
        _ => return Error::BadOp.into(),
    }

    // KeyRegistration::Update updates must also be a valid generation.
    if let KeyRegistration::Update(_, proposed_key_generation) = proposed_key_registration {
        match _validate_key_generation(&validate_data, &proposed_key_generation) {
            Ok(ValidateCallbackResult::Valid) => { },
            validate_callback_result => return validate_callback_result,
        }
    }

    Ok(ValidateCallbackResult::Valid)
}

#[hdk_extern]
/// It is possible to delete a key registration entry IFF the previous element was an update that included a KeyRegistration::Delete
///
/// Currently this MUST be committed immediately after an update KeyRegistration::Delete and the KeyAnchor validation for Delete will check this and must reference this.
///
/// @todo
///  - not sure of the usefulness of this, seems like it could open up optimisations on the get side of things later
///  - an attacker can always NOT include this, so it's intentionally left open for anyone to be able to "heal" a CRUD tree that has not been properly tombstoned
fn validate_delete_entry_key_registration(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    match validate_data.element.action() {
        Action::Delete(delete_header) => {
            match resolve_dependency::<KeyRegistration>(delete_header.deletes_address.clone().into())? {
                Ok(ResolvedDependency(_, prior_key_registration)) => match prior_key_registration {
                    KeyRegistration::Delete(_) => Ok(ValidateCallbackResult::Valid),
                    // Can only Delete a KeyRegistration::Delete from an Update.
                    // Tombstoning logic for that is in the update validation.
                    _ => Error::BadOp.into(),
                },
                Err(validate_callback_result) => Ok(validate_callback_result),
            }
        },
        _ => crate::error::Error::WrongHeader.into(),
    }
}

#[cfg(test)]
pub mod test {
    use hdk::prelude::*;
    use ::fixt::prelude::*;
    use holochain_types::prelude::ValidateDataFixturator;
    use crate::key_registration::validate::KeyRegistration;
    use crate::key_registration::entry::KeyGenerationFixturator;
    use crate::key_registration::entry::KeyRevocationFixturator;
    use holochain_types::prelude::RecordFixturator;
    use crate::generator::entry::GeneratorFixturator;
    use crate::key_registration::error::Error;
    use crate::change_rule::entry::ChangeRuleFixturator;
    use crate::key_registration::validate::KeyRevocation;
    use crate::change_rule::entry::AuthoritySpec;

    #[test]
    fn test_validate_create_entry_key_registration() {
        let mut validate_data = fixt!(ValidateData);
        let key_registration_update = KeyRegistration::Update(fixt!(KeyRevocation), fixt!(KeyGeneration));
        let key_registration_delete = KeyRegistration::Delete(fixt!(KeyRevocation));
        let create_header = fixt!(Create);

        *validate_data.element.as_header_mut() = Action::Create(create_header.clone());

        assert_eq!(
            super::validate_create_entry_key_registration(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_update.clone().try_into().unwrap());

        assert_eq!(
            super::validate_create_entry_key_registration(validate_data.clone()),
            crate::key_registration::error::Error::BadOp.into(),
        );

        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_delete.clone().try_into().unwrap());

        assert_eq!(
            super::validate_create_entry_key_registration(validate_data.clone()),
            crate::key_registration::error::Error::BadOp.into(),
        );

        // See test_validate_key_generation for the valid case and mocking.
        // This is unlike the rest of the repo because the dependency resolution was akward to do inline.
    }

    #[test]
    fn test_validate_update_entry_key_registration() {
        let mut rng = rand::thread_rng();

        let mut validate_data = fixt!(ValidateData);
        let key_revocation = fixt!(KeyRevocation);
        let key_registration_create = KeyRegistration::Create(fixt!(KeyGeneration));
        let key_registration_update = KeyRegistration::Update(key_revocation.clone(), fixt!(KeyGeneration));
        let key_registration_delete = KeyRegistration::Delete(key_revocation.clone());

        // Garbage entry.
        *validate_data.element.as_header_mut() = Action::Update(fixt!(Update));
        *validate_data.element.as_entry_mut() = RecordEntry::NotApplicable;
        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        // Cannot send a KeyRegistration::Create to an update.
        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_create.clone().try_into().unwrap());

        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            crate::key_registration::error::Error::BadOp.into(),
        );

        // Update and delete should work exactly the same apart from the additional generation check for Update.
        if rng.gen::<bool>() {
            *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_update.clone().try_into().unwrap());
        }
        else {
            *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_delete.clone().try_into().unwrap());
        }

        let mut prior_key_registration_element = fixt!(Record);
        let prior_generator = fixt!(Generator);
        let mut prior_generator_element = fixt!(Record);
        *prior_generator_element.as_entry_mut() = RecordEntry::Present(prior_generator.clone().try_into().unwrap());
        let key_generation = fixt!(KeyGeneration);
        let prior_key_registration_delete = KeyRegistration::Delete(fixt!(KeyRevocation));
        let prior_key_registration_create = KeyRegistration::Create(key_generation.clone());
        let prior_key_registration_update = KeyRegistration::Update(fixt!(KeyRevocation), key_generation.clone());

        // Can't "delete" a delete.
        *prior_key_registration_element.as_entry_mut() = RecordEntry::Present(prior_key_registration_delete.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_revocation.as_prior_key_registration_ref().clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![key_revocation.as_prior_key_registration_ref().clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_revocation.as_prior_key_registration_ref().clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(prior_key_registration_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            Error::Tombstone.into(),
        );

        // Can "delete" a create or update exactly the same.
        if rng.gen::<bool>() {
            *prior_key_registration_element.as_entry_mut() = RecordEntry::Present(prior_key_registration_create.clone().try_into().unwrap());
        }
        else {
            *prior_key_registration_element.as_entry_mut() = RecordEntry::Present(prior_key_registration_update.clone().try_into().unwrap());
        }

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_revocation.as_prior_key_registration_ref().clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(prior_key_registration_element.clone())));

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_generation.as_generator_ref().to_owned().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![key_generation.as_generator_ref().to_owned().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_revocation.as_prior_key_registration_ref().clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(prior_key_registration_element.clone())));

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                key_generation.as_generator_ref().to_owned().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(prior_generator_element)));

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                prior_generator.as_change_rule_ref().clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_update_entry_key_registration(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![prior_generator.as_change_rule_ref().clone().into()])),
        );
    }

    #[test]
    fn test_validate_delete_entry_key_registration() {
        let mut validate_data = fixt!(ValidateData);
        let create_header = fixt!(Create);
        let update_header = fixt!(Update);
        let delete_header = fixt!(Delete);
        let key_registration_create = KeyRegistration::Create(fixt!(KeyGeneration));
        let key_registration_update = KeyRegistration::Update(fixt!(KeyRevocation), fixt!(KeyGeneration));
        let key_registration_delete = KeyRegistration::Delete(fixt!(KeyRevocation));

        *validate_data.element.as_header_mut() = Action::Create(create_header);
        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_create.clone().try_into().unwrap());

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            crate::error::Error::WrongHeader.into(),
        );

        *validate_data.element.as_header_mut() = Action::Update(update_header);
        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_update.clone().try_into().unwrap());

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            crate::error::Error::WrongHeader.into(),
        );

        *validate_data.element.as_header_mut() = Action::Delete(delete_header.clone());
        *validate_data.element.as_entry_mut() = RecordEntry::Present(key_registration_delete.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                delete_header.deletes_address.clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![delete_header.deletes_address.clone().into()])),
        );

        let mut return_element = fixt!(Record);

        *return_element.as_header_mut() = Action::Delete(fixt!(Delete));
        *return_element.as_entry_mut() = RecordEntry::NotApplicable;

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                delete_header.deletes_address.clone().into(),
                GetOptions::content()
            )
        ))
        .times(1)
        .return_const(Ok(Some(return_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        *return_element.as_entry_mut() = RecordEntry::Present(key_registration_create.try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                delete_header.deletes_address.clone().into(),
                GetOptions::content()
            )
        ))
        .times(1)
        .return_const(Ok(Some(return_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            Error::BadOp.into(),
        );

        *return_element.as_entry_mut() = RecordEntry::Present(key_registration_update.try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                delete_header.deletes_address.clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(return_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            Error::BadOp.into(),
        );

        *return_element.as_entry_mut() = RecordEntry::Present(key_registration_delete.try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(mockall::predicate::eq(
            GetInput::new(
                delete_header.deletes_address.clone().into(),
                GetOptions::content(),
            )
        ))
        .times(1)
        .return_const(Ok(Some(return_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_delete_entry_key_registration(validate_data.clone()),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    pub fn test_validate_key_generation() {
        let validate_data = fixt!(ValidateData);
        let key_generation = fixt!(KeyGeneration);

        let mut generator_element = fixt!(Record);
        let generator = fixt!(Generator);

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
        .with(mockall::predicate::eq(
            GetInput::new(
                key_generation.as_generator_ref().clone().into(),
                GetOptions::content()
            )
        ))
        .times(1)
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_generation(&validate_data, &key_generation),
            Ok(
                ValidateCallbackResult::UnresolvedDependencies(
                    vec![key_generation.as_generator_ref().clone().into()]
                )
            ),
        );

        *generator_element.as_entry_mut() = RecordEntry::Present(generator.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    key_generation.as_generator_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(Some(generator_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_generation(&validate_data, &key_generation),
            Error::BadAuthor.into(),
        );

        let mut generator_element_header = fixt!(Create);
        generator_element_header.author = validate_data.element.action().author().clone();

        *generator_element.as_header_mut() = Action::Create(generator_element_header);

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    key_generation.as_generator_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(Some(generator_element.clone())));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    key_generation.as_new_key_ref().to_owned(),
                    key_generation.as_new_key_signing_of_author_ref().to_owned(),
                    validate_data.element.action().author().get_raw_32().to_vec(),
                )
            ))
            .times(1)
            .return_const(Ok(false));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_generation(&validate_data, &key_generation),
            Error::BadSelfSignature.into(),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    key_generation.as_generator_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(Some(generator_element.clone())));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    key_generation.as_new_key_ref().to_owned(),
                    key_generation.as_new_key_signing_of_author_ref().to_owned(),
                    validate_data.element.header().author().get_raw_32().to_vec(),
                )
            ))
            .times(1)
            .return_const(Ok(true));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    generator.as_change_ref().as_new_key_ref().to_owned(),
                    key_generation.as_generator_signature_ref().to_owned(),
                    key_generation.as_new_key_ref().as_ref().to_vec(),
                )
            ))
            .times(1)
            .return_const(Ok(false));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_generation(&validate_data, &key_generation),
            Error::BadGeneratorSignature.into(),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    key_generation.as_generator_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(Some(generator_element.clone())));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    key_generation.as_new_key_ref().to_owned(),
                    key_generation.as_new_key_signing_of_author_ref().to_owned(),
                    validate_data.element.header().author().get_raw_32().to_vec(),
                )
            ))
            .times(1)
            .return_const(Ok(true));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new_raw(
                    generator.as_change_ref().as_new_key_ref().to_owned(),
                    key_generation.as_generator_signature_ref().to_owned(),
                    key_generation.as_new_key_ref().as_ref().to_vec(),
                )
            ))
            .times(1)
            .return_const(Ok(true));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_generation(&validate_data, &key_generation),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    pub fn test_validate_key_revocation() {
        let mut validate_data = fixt!(ValidateData);
        let mut prior_key_change_rule = fixt!(ChangeRule);
        let prior_authority_spec = AuthoritySpec {
            sigs_required: 1,
            authorized_signers: vec![fixt!(AgentPubKey)],
        };
        prior_key_change_rule.spec_change.new_spec = prior_authority_spec.clone();

        let revocation_authorization = vec![(0, fixt!(Signature))];
        let key_revocation = KeyRevocation::new(
            fixt!(HeaderHash),
            revocation_authorization.clone(),
        );

        let create_action = fixt!(Create);
        let mut update_action = fixt!(Update);
        let delete_action = fixt!(Delete);

        *validate_data.element.as_action_mut() = Action::Create(create_action.clone());
        assert_eq!(
            super::_validate_key_revocation(&validate_data, &prior_key_change_rule, &key_revocation),
            Error::BadOp.into(),
        );

        *validate_data.element.as_action_mut() = Action::Delete(delete_action.clone());
        assert_eq!(
            super::_validate_key_revocation(&validate_data, &prior_key_change_rule, &key_revocation),
            Error::BadOp.into(),
        );

        *validate_data.element.as_action_mut() = Action::Update(update_action.clone());
        assert_eq!(
            super::_validate_key_revocation(&validate_data, &prior_key_change_rule, &key_revocation),
            Error::BadHeaderRef.into(),
        );

        update_action.original_actio_address = key_revocation.as_prior_key_registration_ref().clone();
        *validate_data.element.as_action_mut() = Action::Update(update_action.clone());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_verify_signature()
        .with(
            mockall::predicate::eq(
                VerifySignature::new_raw(
                    prior_authority_spec.authorized_signers[0].clone(),
                    revocation_authorization[0].1.clone(),
                    key_revocation.as_prior_key_registration_ref().get_raw_32().to_vec()
                )
            )
        )
        .times(1)
        .return_const(Ok(false));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_revocation(&validate_data, &prior_key_change_rule, &key_revocation),
            crate::change_rule::error::Error::BadUpdateSignature.into(),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_verify_signature()
        .with(
            mockall::predicate::eq(
                VerifySignature::new_raw(
                    prior_authority_spec.authorized_signers[0].clone(),
                    revocation_authorization[0].1.clone(),
                    key_revocation.as_prior_key_registration_ref().get_raw_32().to_vec()
                )
            )
        )
        .times(1)
        .return_const(Ok(true));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_key_revocation(&validate_data, &prior_key_change_rule, &key_revocation),
            Ok(ValidateCallbackResult::Valid),
        );
    }
}
