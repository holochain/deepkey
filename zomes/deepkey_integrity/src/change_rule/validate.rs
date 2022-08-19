use hdk::prelude::*;
use crate::change_rule::error::Error;
use crate::change_rule::entry::ChangeRule;
use crate::keyset_root::entry::KeysetRoot;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::entry::UnitEntryTypes;

use crate::validate_classic::*;

fn _validate_keyset_leaf(validate_data: &ValidateData, change_rule: &ChangeRule) -> ExternResult<ValidateCallbackResult> {
    let leaf_header_element: Record = match get(change_rule.as_keyset_leaf_ref().clone(), GetOptions::content())? {
        Some(element) => element,
        None => return Ok(ValidateCallbackResult::UnresolvedDependencies(
            UnresolvedDependencies::Hashes(vec![change_rule.as_keyset_leaf_ref().clone().into()]))),
    };

    // The leaf MUST be a device acceptance if not the root itself.
    if change_rule.keyset_root != change_rule.keyset_leaf {
        // so it MUST deserialize cleanly
        let device_invite_acceptance = match DeviceInviteAcceptance::try_from(&leaf_header_element) {
            Ok(device_invite_acceptance) => device_invite_acceptance,
            Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
        };
        // and the keyset root MUST be the same on the leaf and change rule
        if change_rule.as_keyset_root_ref() != device_invite_acceptance.as_keyset_root_authority_ref() {
            return Error::BadKeysetLeaf.into();
        }
    }
    else {
        // We already validate the keyset root as a root elsewhere so nothing to do here...
    }

    // @todo - way to do this without full chain validation package?
    let info = zome_info()?;
    let zome_id = info.id;
    // TODO: I can't figure out how to get an EntryType::App(AppEntryType{ ... }) from any of this
    // Zome's Entry Types, to compare against an action().entry_type().  It seems very
    // ... difficult.
    let dia_index: EntryDefIndex = UnitEntryTypes::DeviceInviteAcceptance.try_into().unwrap();
    let device_invite_acceptance_type = EntryType::App(AppEntryType::new(
        dia_index, zome_id, EntryVisibility::Public,
    ));
    
    match &validate_data.validation_package {
        Some(ValidationPackage(elements)) => {
            //let device_invite_acceptance_type = UnitEntryTypes::DeviceInviteAcceptance; //entry_type!(DeviceInviteAcceptance)?;
            let device_invite_acceptances: Vec<&Record> = elements.iter()
                .filter(|element| element.action().entry_type() == Some(&device_invite_acceptance_type))
                .filter(|element| element.action().action_seq() >= leaf_header_element.action().action_seq())
                .collect();
            if device_invite_acceptances.len() != 1 {
                return Error::StaleKeysetLeaf.into();
            }
        },
        None => return Error::MissingValidationPackage.into(),
    }

    Ok(ValidateCallbackResult::Valid)
}


fn _validate_spec(change_rule: &ChangeRule) -> ExternResult<ValidateCallbackResult> {
    if change_rule.spec_change.new_spec.sigs_required as usize > change_rule.spec_change.new_spec.authorized_signers.len() {
        Error::NotEnoughSigners.into()
    }
    else if change_rule.spec_change.new_spec.sigs_required < 1 {
            Error::NotEnoughSignatures.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_create_keyset_root(validate_data: &ValidateData, change_rule: &ChangeRule, keyset_root: &KeysetRoot) -> ExternResult<ValidateCallbackResult> {
    // The KSR needs to reference the author as the FDA.
    if keyset_root.as_first_deepkey_agent_ref() != validate_data.element.action().author() {
        return Error::AuthorNotFda.into()
    }

    // Create must be immediately after KeysetRoot.
    if validate_data.element.action().prev_action() != Some(change_rule.as_keyset_root_ref()) {
        return Error::CreateNotAfterKeysetRoot.into()
    }

    Ok(ValidateCallbackResult::Valid)
}

fn _validate_create_authorization(_: &ValidateData, change_rule: &ChangeRule, keyset_root: &KeysetRoot) -> ExternResult<ValidateCallbackResult> {
    // Signed by throwaway RootKey on Create, must have exactly one signature.
    if change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().len() > 1 {
        return Error::MultipleCreateSignatures.into();
    }
    let authorization_signature = match change_rule.as_spec_change_ref().as_authorization_of_new_spec_ref().iter().next() {
        Some(signature) => &signature.1,
        None => return Error::NoCreateSignature.into(),
    };

    // The signature must be valid.
    if verify_signature(
        keyset_root.as_root_pub_key_ref().clone(),
        authorization_signature.clone(),
        change_rule.as_spec_change_ref().as_new_spec_ref()
    )? {
        Ok(ValidateCallbackResult::Valid)
    } else {
        Error::BadCreateSignature.into()
    }
}

#[hdk_extern]
fn validate_create_entry_key_change_rule(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_change_rule = match ChangeRule::try_from(&validate_data.element) {
        Ok(change_rule) => change_rule,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    let keyset_root: KeysetRoot = match resolve_dependency(proposed_change_rule.as_keyset_root_ref().clone().into())? {
        Ok(ResolvedDependency(_, keyset_root)) => keyset_root,
        Err(validate_callback_result) => return Ok(validate_callback_result),
    };

    match _validate_keyset_leaf(&validate_data, &proposed_change_rule) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
    }

    match _validate_create_keyset_root(&validate_data, &proposed_change_rule, &keyset_root) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
    }

    match _validate_create_authorization(&validate_data, &proposed_change_rule, &keyset_root) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
    }

    match _validate_spec(&proposed_change_rule) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
    }

    Ok(ValidateCallbackResult::Valid)
}

fn _validate_update_keyset_root(_: &ValidateData, previous_change_rule: &ChangeRule, proposed_change_rule: &ChangeRule) -> ExternResult<ValidateCallbackResult> {
    // The keyset root needs to be the same
    if proposed_change_rule.as_keyset_root_ref() != previous_change_rule.as_keyset_root_ref() {
        Error::KeysetRootMismatch.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_update_authorization(_: &ValidateData, previous_change_rule: &ChangeRule, proposed_change_rule: &ChangeRule) -> ExternResult<ValidateCallbackResult> {
    match previous_change_rule.authorize(&proposed_change_rule.spec_change.authorization_of_new_spec,
                                         &holochain_serialized_bytes::encode(&proposed_change_rule.spec_change.new_spec)?) {
        Ok(_) => Ok(ValidateCallbackResult::Valid),
        Err(e) => Ok(e.into()), // converts change_rule::error::Error to Invalid callback result w/ string description
    }
}

// We want a flat CRUD tree so that get_details on the first change rule returns all the subsequent change rules.
fn _validate_flat_update_tree(previous_change_rule_element: &Record)  -> ExternResult<ValidateCallbackResult> {
    // The previous change rule MUST be the root of the CRUD tree.
    // i.e. the updates MUST always point to the original Create header (not an Update).
    // The create validation ensures that it always immediately follows the KeysetRoot
    match previous_change_rule_element.action() {
        Action::Create(_) => Ok(ValidateCallbackResult::Valid),
        _ => Error::BranchingUpdates.into(),
    }
}

#[hdk_extern]
fn validate_update_entry_key_change_rule(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let proposed_change_rule = match ChangeRule::try_from(&validate_data.element) {
        Ok(change_rule) => change_rule,
        Err(e) => return e.into(),
    };

    // KeysetRoot needs to exist.
    match resolve_dependency::<KeysetRoot>(proposed_change_rule.as_keyset_root_ref().clone().into())? {
        Err(validate_callback_result) => return Ok(validate_callback_result),
        _ => { },
    }

    // On update we need to validate the proposed change rule under the rules of the previous rule.
    match validate_data.element.action() {
        Action::Update(update_header) => {
            let (previous_change_rule_element, previous_change_rule) = match resolve_dependency::<ChangeRule>(update_header.original_header_address.clone().into())? {
                Ok(ResolvedDependency(previous_change_rule_element, change_rule)) => (previous_change_rule_element, change_rule),
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };

            match _validate_flat_update_tree(&previous_change_rule_element) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            match _validate_keyset_leaf(&validate_data, &proposed_change_rule) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            match _validate_update_keyset_root(&validate_data, &previous_change_rule, &proposed_change_rule) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            match _validate_update_authorization(&validate_data, &previous_change_rule, &proposed_change_rule) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            match _validate_spec(&proposed_change_rule) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            Ok(ValidateCallbackResult::Valid)
        },
        Action::Delete(_) => Error::DeleteAttempted.into(),
        _ => Error::WrongHeader.into(),
    }
}

#[hdk_extern]
fn validate_delete_entry_key_change_rule(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use ::fixt::prelude::*;
    use holochain_types::prelude::*;
    use crate::change_rule::entry::ChangeRuleFixturator;
    use crate::keyset_root::entry::KeysetRootFixturator;
    use crate::change_rule::error::Error;
    use crate::change_rule::entry::AuthorizationFixturator;
    use crate::change_rule::entry::Authorization;
    use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptanceFixturator;
    use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;

    #[test]
    fn test_validate_keyset_leaf() {
        let mut validate_data = fixt!(ValidateData);
        let mut validate_header = fixt!(Update);
        validate_header.header_seq = 50;

        *validate_data.element.as_header_mut() = Action::Update(validate_header);

        let change_rule = fixt!(ChangeRule);
        let mut device_invite_acceptance = fixt!(DeviceInviteAcceptance);
        let mut device_invite_acceptance_element = fixt!(Record);

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![change_rule.as_keyset_leaf_ref().clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        set_hdk(mock_hdk);

        *device_invite_acceptance_element.as_header_mut() = Action::Update(fixt!(Update));
        *device_invite_acceptance_element.as_entry_mut() = RecordEntry::Present(device_invite_acceptance.clone().try_into().unwrap());

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            crate::error::Error::WrongHeader.into(),
        );

        let mut device_invite_element_header = fixt!(Create);

        let mut mock_hdk = MockHdkT::new();
        let zome_info = fixt!(ZomeInfo);
        mock_hdk.expect_zome_info().return_const(Ok(zome_info.clone()));
        set_hdk(mock_hdk);
        device_invite_element_header.entry_type = entry_type!(DeviceInviteAcceptance).unwrap();

        device_invite_element_header.header_seq = 25;
        *device_invite_acceptance_element.as_header_mut() = Action::Create(device_invite_element_header.clone());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Error::BadKeysetLeaf.into(),
        );

        device_invite_acceptance.keyset_root_authority = change_rule.as_keyset_root_ref().clone();
        *device_invite_acceptance_element.as_entry_mut() = RecordEntry::Present(device_invite_acceptance.clone().try_into().unwrap());

        validate_data.validation_package = None;

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Error::MissingValidationPackage.into(),
        );

        // newer device acceptance
        let mut newer_device_invite_acceptance_element = device_invite_acceptance_element.clone();
        let mut newer_device_invite_acceptance_header = device_invite_element_header.clone();
        newer_device_invite_acceptance_header.header_seq = 30;
        *newer_device_invite_acceptance_element.as_header_mut() = Action::Create(newer_device_invite_acceptance_header);
        validate_data.validation_package = Some(ValidationPackage(
            vec![fixt!(Record), device_invite_acceptance_element.clone(), fixt!(Record), newer_device_invite_acceptance_element]
        ));

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        mock_hdk.expect_zome_info().return_const(Ok(zome_info.clone()));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Error::StaleKeysetLeaf.into(),
        );

        // nothing newer

        validate_data.validation_package = Some(ValidationPackage(
            vec![fixt!(Record), device_invite_acceptance_element.clone(), fixt!(Record)]
        ));

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        mock_hdk.expect_zome_info().return_const(Ok(zome_info.clone()));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Ok(ValidateCallbackResult::Valid),
        );

        // something older nothing newer

        let mut older_device_invite_acceptance_element = device_invite_acceptance_element.clone();
        let mut older_device_invite_acceptance_header = device_invite_element_header.clone();
        older_device_invite_acceptance_header.header_seq = 10;
        *older_device_invite_acceptance_element.as_header_mut() = Action::Create(older_device_invite_acceptance_header);
        validate_data.validation_package = Some(ValidationPackage(
            vec![older_device_invite_acceptance_element.clone(), device_invite_acceptance_element.clone()]
        ));

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get().with(
            mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_leaf_ref().clone().into(),
                    GetOptions::content()
                )
            )
        )
        .return_const(Ok(Some(device_invite_acceptance_element.clone())));

        mock_hdk.expect_zome_info().return_const(Ok(zome_info.clone()));

        set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_keyset_leaf(&validate_data, &change_rule),
            Ok(ValidateCallbackResult::Valid)
        );
    }

    #[test]
    fn test_validate_update() {
        // Random garbage won't have a valid ChangeRule on it.
        assert_eq!(
            super::validate_update_entry_key_change_rule(fixt!(ValidateData)),
            Ok(ValidateCallbackResult::Invalid("Record missing its ChangeRule".to_string())),
        );

        let mut validate_data = fixt!(ValidateData);
        let mut change_rule = fixt!(ChangeRule);
        // Ensure at least one signer.
        change_rule.spec_change.new_spec.authorized_signers.push(fixt!(AgentPubKey));

        let update_header = fixt!(Update);
        *validate_data.element.as_header_mut() = Action::Update(update_header.clone());

        let mut keyset_root_element = fixt!(Record);
        let keyset_root = fixt!(KeysetRoot);
        *keyset_root_element.as_entry_mut() = RecordEntry::Present(keyset_root.clone().try_into().unwrap());

        *validate_data.element.as_entry_mut() = RecordEntry::Present(change_rule.clone().try_into().unwrap());

        let previous_change_rule = fixt!(ChangeRule);
        let mut previous_element = fixt!(Record);
        *previous_element.as_entry_mut() = RecordEntry::Present(previous_change_rule.clone().try_into().unwrap());

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
            super::validate_update_entry_key_change_rule(validate_data.clone()),
        );

        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    change_rule.as_keyset_root_ref().clone().into(),
                    GetOptions::content()
                )
            ))
            .times(1)
            .return_const(Ok(Some(keyset_root_element.clone())));

        mock_hdk.expect_get()
            .with(mockall::predicate::eq(
                GetInput::new(
                    update_header.original_header_address.clone().into(),
                    GetOptions::content(),
                )
            ))
            .times(1)
            .return_const(Ok(None));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            Ok(
                ValidateCallbackResult::UnresolvedDependencies(vec![update_header.original_header_address.clone().into()])
            ),
            super::validate_update_entry_key_change_rule(validate_data.clone()),
        );
    }

    #[test]
    fn test_validate_delete() {
        assert_eq!(
            super::validate_delete_entry_key_change_rule(fixt!(ValidateData)),
            Error::DeleteAttempted.into(),
        );
    }

    #[test]
    fn test_validate_create() {
        // Random garbage won't have a valid ChangeRule on it.
        assert_eq!(
            Ok(ValidateCallbackResult::Invalid("Record missing its ChangeRule".to_string())),
            super::validate_create_entry_key_change_rule(fixt!(ValidateData)),
        );

        let mut validate_data = fixt!(ValidateData);
        let change_rule = fixt!(ChangeRule);

        *validate_data.element.as_entry_mut() = RecordEntry::Present(change_rule.clone().try_into().unwrap());

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
    fn test_validate_spec() {
        let mut change_rule = fixt!(ChangeRule);

        change_rule.spec_change.new_spec.sigs_required = 200;

        assert_eq!(
            super::_validate_spec(&change_rule),
            Error::NotEnoughSigners.into(),
        );

        change_rule.spec_change.new_spec.sigs_required = 0;

        assert_eq!(
            super::_validate_spec(&change_rule),
            Error::NotEnoughSignatures.into(),
        );

        change_rule.spec_change.new_spec.authorized_signers.push(fixt!(AgentPubKey));
        change_rule.spec_change.new_spec.sigs_required = change_rule.spec_change.new_spec.authorized_signers.len() as u8;
        assert_eq!(
            super::_validate_spec(&change_rule),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_keyset_root() {
        let mut validate_data = fixt!(ValidateData);
        let change_rule = fixt!(ChangeRule);
        let keyset_root = fixt!(KeysetRoot);
        let mut create_header = fixt!(Create);

        *validate_data.element.as_header_mut() = Action::Create(create_header.clone());

        // The FDA cannot be valid unless the validation element and keyset root FDA are the same.
        assert_eq!(
            super::_validate_create_keyset_root(&validate_data, &change_rule, &keyset_root),
            Error::AuthorNotFda.into(),
        );

        create_header.author = keyset_root.as_first_deepkey_agent_ref().clone();
        *validate_data.element.as_header_mut() = Action::Create(create_header.clone());

        assert_eq!(
            super::_validate_create_keyset_root(&validate_data, &change_rule, &keyset_root),
            Error::CreateNotAfterKeysetRoot.into(),
        );

        create_header.prev_header = change_rule.as_keyset_root_ref().clone();
        *validate_data.element.as_header_mut() = Action::Create(create_header);

        assert_eq!(
            super::_validate_create_keyset_root(&validate_data, &change_rule, &keyset_root),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_authorization() {
        let validate_data = fixt!(ValidateData);
        let mut change_rule = fixt!(ChangeRule);
        let keyset_root = fixt!(KeysetRoot);

        change_rule.spec_change.authorization_of_new_spec.push(fixt!(Authorization));
        change_rule.spec_change.authorization_of_new_spec.push(fixt!(Authorization));

        // Too many sigs fails.
        assert_eq!(
            super::_validate_create_authorization(&validate_data, &change_rule, &keyset_root),
            Error::MultipleCreateSignatures.into(),
        );

        change_rule.spec_change.authorization_of_new_spec = vec![];

        // No sig fails.
        assert_eq!(
            super::_validate_create_authorization(&validate_data, &change_rule, &keyset_root),
            Error::NoCreateSignature.into(),
        );

        // Invalid sig fails.
        let authorization = fixt!(Authorization);
        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(VerifySignature::new(
                keyset_root.as_root_pub_key_ref().clone(),
                authorization.1.clone(),
                change_rule.as_spec_change_ref().as_new_spec_ref().clone()
            ).unwrap()))
            .times(1)
            .return_const(Ok(false));

        hdk::prelude::set_hdk(mock_hdk);

        change_rule.spec_change.authorization_of_new_spec = vec![authorization];

        assert_eq!(
            super::_validate_create_authorization(&validate_data, &change_rule, &keyset_root),
            Error::BadCreateSignature.into(),
        );

        // Valid sig passes.
        let mut mock_hdk = hdk::prelude::MockHdkT::new();
        mock_hdk.expect_verify_signature()
            .times(1)
            .return_const(Ok(true));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            Ok(ValidateCallbackResult::Valid),
            super::_validate_create_authorization(&validate_data, &change_rule, &keyset_root),
        );
    }

    #[test]
    fn test_validate_update_keyset_root() {
        let validate_data = fixt!(ValidateData);
        let previous_change_rule = fixt!(ChangeRule);
        let mut proposed_change_rule = fixt!(ChangeRule);

        assert_eq!(
            super::_validate_update_keyset_root(&validate_data, &previous_change_rule, &proposed_change_rule),
            Error::KeysetRootMismatch.into(),
        );

        proposed_change_rule.keyset_root = previous_change_rule.keyset_root.clone();

        assert_eq!(
            super::_validate_update_keyset_root(&validate_data, &previous_change_rule, &proposed_change_rule),
            Ok(ValidateCallbackResult::Valid),
        )
    }

    #[test]
    fn test_validate_update_authorization() {
        let validate_data = fixt!(ValidateData);
        let mut previous_change_rule = fixt!(ChangeRule);
        let mut proposed_change_rule = fixt!(ChangeRule);

        // Add a couple of signatures to make tests easier to write.
        proposed_change_rule.spec_change.authorization_of_new_spec.push(fixt!(Authorization));
        proposed_change_rule.spec_change.authorization_of_new_spec.push(fixt!(Authorization));

        // Fewer signatures than required is a fail.
        previous_change_rule.spec_change.new_spec.sigs_required = proposed_change_rule.spec_change.authorization_of_new_spec.len() as u8 + 1;

        assert_eq!(
            super::_validate_update_authorization(&validate_data, &previous_change_rule, &proposed_change_rule),
            Error::WrongNumberOfSignatures.into(),
        );

        // More signatures than required is also a fail!
        previous_change_rule.spec_change.new_spec.sigs_required = proposed_change_rule.spec_change.authorization_of_new_spec.len() as u8 - 1;

        assert_eq!(
            super::_validate_update_authorization(&validate_data, &previous_change_rule, &proposed_change_rule),
            Error::WrongNumberOfSignatures.into(),
        );

        let three_signers: Vec<AgentPubKey> = AgentPubKeyFixturator::new(Predictable).take(3).collect();

        previous_change_rule.spec_change.new_spec.authorized_signers = three_signers.clone();
        previous_change_rule.spec_change.new_spec.sigs_required = 2;

        let two_signatures: Vec<Authorization> = vec![(0, fixt!(Signature)), (2, fixt!(Signature))];
        proposed_change_rule.spec_change.authorization_of_new_spec = two_signatures.clone();
        proposed_change_rule.spec_change.new_spec.sigs_required = 2;

        // Bad signatures is a fail.
        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new(
                    three_signers[0].clone().into(),
                    two_signatures[0].1.clone().into(),
                    proposed_change_rule.spec_change.new_spec.clone(),
                ).unwrap()
            ))
            .times(1)
            .return_const(Ok(true));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new(
                    three_signers[2].clone().into(),
                    two_signatures[1].1.clone().into(),
                    proposed_change_rule.spec_change.new_spec.clone(),
                ).unwrap()
            ))
            .times(1)
            .return_const(Ok(false));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_update_authorization(&validate_data, &previous_change_rule, &proposed_change_rule),
            Error::BadUpdateSignature.into(),
        );

        // All sigs valid = pass.
        let mut mock_hdk = hdk::prelude::MockHdkT::new();

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new(
                    three_signers[0].clone().into(),
                    two_signatures[0].1.clone().into(),
                    proposed_change_rule.spec_change.new_spec.clone(),
                ).unwrap()
            ))
            .times(1)
            .return_const(Ok(true));

        mock_hdk.expect_verify_signature()
            .with(mockall::predicate::eq(
                VerifySignature::new(
                    three_signers[2].clone().into(),
                    two_signatures[1].1.clone().into(),
                    proposed_change_rule.spec_change.new_spec.clone(),
                ).unwrap()
            ))
            .times(1)
            .return_const(Ok(true));

        hdk::prelude::set_hdk(mock_hdk);

        assert_eq!(
            super::_validate_update_authorization(&validate_data, &previous_change_rule, &proposed_change_rule),
            Ok(ValidateCallbackResult::Valid),
        );
    }
}
