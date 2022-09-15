use hdk::prelude::*;
use crate::device_authorization::device_invite::error::Error;
use crate::validate::ResolvedDependency;
use crate::validate::resolve_dependency;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::entry::UnitEntryTypes;
use crate::validate_classic::*;

fn _validate_self(create_header: &Create, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    // Cannot self-invite.
    // Note: A device _MAY_ still be referenced multiple times from a branching tree of invites.
    if &create_header.author == device_invite.as_device_agent_ref() {
        return Error::SelfInvite.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

fn _validate_parent_current(validate_data: &ValidateData, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    let parent_element: Record = must_get_valid_record(device_invite.as_parent_ref().clone())?;
    match &validate_data.validation_package {
        Some(ValidationPackage(elements)) => {
            // This used the entry_type! macro, producing an Error<App(AppEntryType)>
            // #[macro_export]
            // macro_rules! entry_type {
            //     ( $t:ty ) => {
            //         match $crate::prelude::entry_def_index!($t) {
            //             Ok(e_id) => match $crate::prelude::zome_info() {
            //                 Ok(ZomeInfo { id, .. }) => Ok($crate::prelude::EntryType::App(
            //                     $crate::prelude::AppEntryType::new(e_id, id, <$t>::entry_visibility()),
            //                 )),
            //                 Err(e) => Err(e),
            //                 _ => unreachable!(),
            //             },
            //             Err(e) => Err(e),
            //         }
            //     };
            // }
            // Now, we use the Enum resulting from the unit_enum(...) macro, which contains all of the known EntryType::App(AppEntryType)s:
            //
            // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SerializedBytes, Hash)]
            // #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
            // pub enum EntryType {
            //     /// An AgentPubKey
            //     AgentPubKey,
            //     /// An app-provided entry, along with its app-provided AppEntryType
            //     App(AppEntryType),
            //     /// A Capability claim
            //     CapClaim,
            //     /// A Capability grant.
            //     CapGrant,
            // }
            //
            // /// Information about a class of Entries provided by the DNA
            // #[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, SerializedBytes, Hash)]
            // #[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
            // pub struct AppEntryType {
            //     /// A unique u8 identifier within a zome for this
            //     /// entry type.
            //     pub id: EntryDefIndex,
            //     /// The id of the zome that defines this entry type.
            //     pub zome_id: ZomeId,
            //     // @todo don't do this, use entry defs instead
            //     /// The visibility of this app entry.
            //     pub visibility: EntryVisibility,
            // }
            // 
            // So, we should be able to substitute a named Enum attribute for the results of the entry_type! macro:
	    // Lets find the EntryTypeIndex and ZomeId of the target Entry type:
	    let dia_scoped_index: ScopedEntryDefIndex = UnitEntryTypes::DeviceInviteAcceptance.try_into().unwrap();
            let device_invite_acceptance_type = EntryType::App(AppEntryType::new(
	            dia_scoped_index.zome_type, dia_scoped_index.zome_id, EntryVisibility::Public,
	    ));//entry_type!(DeviceInviteAcceptance)?;

            let device_invite_acceptances: Vec<&Record> = elements.iter()
                .filter(|element| element.action().entry_type() == Some(&device_invite_acceptance_type))
                .filter(|element| element.action().action_seq() >= parent_element.action().action_seq())
                .collect();
            debug!("_validate_parent_current processed {} source-chain elements to find {} DeviceInviteAcceptances",
                   elements.len(), device_invite_acceptances.len());
            if parent_element.action().entry_type() == Some(&device_invite_acceptance_type) {
                // The parent should be found and nothing else.
                if device_invite_acceptances.len() != 1 {
                    return Error::StaleKeysetLeaf.into();
                }
                if *device_invite_acceptances[0] != parent_element {
                    return Error::StaleKeysetLeaf.into();
                }
            } else {
                // The parent is the KSR so nothing should be found.
                if device_invite_acceptances.len() != 0 {
                    return Error::StaleKeysetLeaf.into();
                }
            };
        },
        None => return Error::MissingValidationPackage.into(),
    }

    Ok(ValidateCallbackResult::Valid)
}

/// If the parent _is_ the KSRA then it gets special treatment.
/// All we care about is that the invitor is the FDA, in which case they can invite any device.
fn _validate_create_parent_ksr(create_header: &Create, parent: &KeysetRoot, _: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    // We know the parent author == parent.first_deepkey_agent as per KeysetRoot validation.
    if create_header.author != parent.first_deepkey_agent {
        Error::AuthorNotFda.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

/// If the parent references an invite, we mostly care that the ultimate KSR is a match.
/// We also care that the current invitor is the parent's invitee.
fn _validate_create_parent_device_invite_acceptance(create_header: &Create, parent_invite: &DeviceInvite, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    if parent_invite.as_keyset_root_authority_ref() != device_invite.as_keyset_root_authority_ref() {
        Error::WrongKeysetRoot.into()
    }
    else if parent_invite.as_device_agent_ref() != &create_header.author {
        Error::WrongAuthor.into()
    }
    else {
        Ok(ValidateCallbackResult::Valid)
    }
}

#[hdk_extern]
fn validate_create_entry_device_invite(validate_data: ValidateData) -> ExternResult<ValidateCallbackResult> {
    let device_invite = match DeviceInvite::try_from(&validate_data.element) {
        Ok(device_invite) => device_invite,
        Err(e) => return Ok(ValidateCallbackResult::Invalid(e.to_string())),
    };

    match confirm_action_device_invite(validate_data.element.action(), &device_invite) {
        Ok(ValidateCallbackResult::Valid) => { },
        validate_callback_result => return validate_callback_result,
    }
    // We are now certain this was a valid Action::Create.
    _validate_parent_current(&validate_data, &device_invite)
}

/// Obtains (Record, Vec<RegisterAgentActivity>) containing all prior Actions, converts to
/// ValidateData and invokes classic validate_create_... method.
pub fn confirm_chain_device_invite(signed_action: SignedHashed<Action>) -> ExternResult<ValidateCallbackResult> {
    debug!("confirm_chain_device_input: {:?}", signed_action);
    let entry_hash: EntryHash = match signed_action.hashed.content.entry_hash() {
        Some(eh) => eh.to_owned(),
        None => return crate::error::Error::EntryMissing.into(),
    };
    let author = signed_action.hashed.content.author().clone();
    let prev_action: ActionHash = match signed_action.hashed.content.prev_action() {
        Some(ah) => ah.to_owned(),
        None => return crate::error::Error::EntryMissing.into(),
    };
    let entry = RecordEntry::Present(must_get_entry( entry_hash )?.content);
    let record = Record{ signed_action, entry };
    let chain: Vec<RegisterAgentActivity> = must_get_agent_activity(author, ChainFilter::new(prev_action))?;
    let validate_data: ValidateData = (record, chain).try_into()?;
    let result = validate_create_entry_device_invite(validate_data);
    debug!("confirm_chain_device_input result: {:?}", result);
    result
}

pub fn confirm_action_device_invite(action: &Action, device_invite: &DeviceInvite) -> ExternResult<ValidateCallbackResult> {
    match action {
        Action::Create(create_header) => {
            match _validate_self(&create_header, device_invite) {
                Ok(ValidateCallbackResult::Valid) => { },
                validate_callback_result => return validate_callback_result,
            }

            let keyset_root_authority: KeysetRoot = match resolve_dependency(device_invite.as_keyset_root_authority_ref().to_owned().into())? {
                Ok(ResolvedDependency(_, keyset_root_authority)) => keyset_root_authority,
                Err(validate_callback_result) => return Ok(validate_callback_result),
            };
            // Note that we do _NOT_ check that the `device_agent` resolves because it may not exist yet.
            // It is valid for a device to join the DHT with a reference to an invite as a joining proof.
            // The `DeviceInviteAcceptance` entry will validate the referential integrity of the `DeviceInvite`.

            if device_invite.as_keyset_root_authority_ref() == device_invite.as_parent_ref() {
                _validate_create_parent_ksr(&create_header, &keyset_root_authority, &device_invite)
            } else {
                let parent: DeviceInviteAcceptance = match resolve_dependency(device_invite.as_parent_ref().to_owned().into())? {
                    Ok(ResolvedDependency(_, device_invite_acceptance)) => device_invite_acceptance,
                    Err(validate_callback_result) => return Ok(validate_callback_result),
                };
                let parent_invite: DeviceInvite = match resolve_dependency(parent.as_invite_ref().to_owned().into())? {
                    Ok(ResolvedDependency(_, device_invite)) => device_invite,
                    Err(validate_callback_result) => return Ok(validate_callback_result),
                };
                _validate_create_parent_device_invite_acceptance(&create_header, &parent_invite, &device_invite)
            }
        },
        Action::Update(_) => Error::UpdateAttempted.into(),
        Action::Delete(_) => Error::DeleteAttempted.into(),
        _ => Error::WrongHeader.into(),
    }
}

#[hdk_extern]
/// Updates are not allowed for DeviceInvite.
fn validate_update_entry_device_invite(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::UpdateAttempted.into()
}

#[hdk_extern]
/// Deletes are not allowed for DeviceInvite.
fn validate_delete_entry_device_invite(_: ValidateData) -> ExternResult<ValidateCallbackResult> {
    Error::DeleteAttempted.into()
}

#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use crate::device_authorization::device_invite::error::Error;
    use ::fixt::prelude::*;
    use holochain_types::prelude::ValidateDataFixturator;
    use holochain_types::prelude::*;
    use crate::device_authorization::device_invite::entry::DeviceInviteFixturator;
    use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptanceFixturator;
    use crate::keyset_root::entry::KeysetRootFixturator;

    #[test]
    fn test_validate_update_entry_device_invite() {
        assert_eq!(
            super::validate_update_entry_device_invite(fixt!(ValidateData)),
            Error::UpdateAttempted.into(),
        );
    }

    #[test]
    fn test_validate_delete_entry_device_invite() {
        assert_eq!(
            super::validate_delete_entry_device_invite(fixt!(ValidateData)),
            Error::DeleteAttempted.into(),
        );
    }

    #[test]
    fn test_validate_self() {
        let create_header = fixt!(Create);
        let mut device_invite = fixt!(DeviceInvite);

        assert_eq!(
            super::_validate_self(&create_header, &device_invite),
            Ok(ValidateCallbackResult::Valid),
        );

        device_invite.device_agent = create_header.author.clone();

        assert_eq!(
            super::_validate_self(&create_header, &device_invite),
            Error::SelfInvite.into(),
        );
    }

    #[test]
    fn test_validate_create_parent_ksr() {
        let mut create_header = fixt!(Create);
        let parent = fixt!(KeysetRoot);
        let device_invite = fixt!(DeviceInvite);

        assert_eq!(
            super::_validate_create_parent_ksr(&create_header, &parent, &device_invite),
            Error::AuthorNotFda.into(),
        );

        create_header.author = parent.as_first_deepkey_agent_ref().to_owned();

        assert_eq!(
            super::_validate_create_parent_ksr(&create_header, &parent, &device_invite),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_parent_device_invite_acceptance() {
        let mut create_header = fixt!(Create);
        let parent_invite = fixt!(DeviceInvite);
        let mut device_invite = fixt!(DeviceInvite);

        assert_eq!(
            super::_validate_create_parent_device_invite_acceptance(&create_header, &parent_invite, &device_invite),
            Error::WrongKeysetRoot.into(),
        );

        device_invite.keyset_root_authority = parent_invite.as_keyset_root_authority_ref().clone();

        assert_eq!(
            super::_validate_create_parent_device_invite_acceptance(&create_header, &parent_invite, &device_invite),
            Error::WrongAuthor.into(),
        );

        create_header.author = parent_invite.as_device_agent_ref().clone();

        assert_eq!(
            super::_validate_create_parent_device_invite_acceptance(&create_header, &parent_invite, &device_invite),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_entry_device_invite_ksr_parent() {
        let mut validate_data = fixt!(ValidateData);
        let mut create_header = fixt!(Create);
        let keyset_root_authority = fixt!(KeysetRoot);
        let mut keyset_root_authority_element = fixt!(Record);
        *keyset_root_authority_element.as_entry_mut() = RecordEntry::Present(keyset_root_authority.clone().try_into().unwrap());
        let mut device_invite = fixt!(DeviceInvite);

        device_invite.keyset_root_authority = device_invite.parent.clone();
        create_header.author = keyset_root_authority.as_first_deepkey_agent_ref().clone();

        *validate_data.element.as_header_mut() = Action::Create(create_header);
        validate_data.validation_package = Some(ValidationPackage(vec![fixt!(Record)]));

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        *validate_data.element.as_entry_mut() = RecordEntry::Present(device_invite.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.keyset_root_authority.clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_invite.keyset_root_authority.clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.keyset_root_authority.clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(2)
            .return_const(Ok(Some(keyset_root_authority_element.clone())));

        mock_hdk.expect_zome_info().times(1).return_const(Ok(fixt!(ZomeInfo)));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::Valid),
        );
    }

    #[test]
    fn test_validate_create_entry_device_invite_acceptance_parent() {
        let mut validate_data = fixt!(ValidateData);
        let keyset_root = fixt!(KeysetRoot);
        let mut keyset_root_authority_element = fixt!(Record);
        *keyset_root_authority_element.as_entry_mut() = RecordEntry::Present(keyset_root.clone().try_into().unwrap());
        let mut create_header = fixt!(Create);
        let parent = fixt!(DeviceInviteAcceptance);
        let device_invite = fixt!(DeviceInvite);
        let mut parent_invite = fixt!(DeviceInvite);
        let zome_info = fixt!(ZomeInfo);

        parent_invite.keyset_root_authority = device_invite.keyset_root_authority.clone();
        create_header.author = parent_invite.device_agent.clone();

        let mut parent_element = fixt!(Record);
        *parent_element.as_entry_mut() = RecordEntry::Present(parent.clone().try_into().unwrap());

        let mut parent_invite_element = fixt!(Record);
        *parent_invite_element.as_entry_mut() = RecordEntry::Present(parent_invite.clone().try_into().unwrap());

        *validate_data.element.as_header_mut() = Action::Create(create_header);

        validate_data.validation_package = Some(ValidationPackage(vec![parent_invite_element.clone()]));

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            crate::error::Error::EntryMissing.into(),
        );

        *validate_data.element.as_entry_mut() = RecordEntry::Present(device_invite.clone().try_into().unwrap());

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.keyset_root_authority.clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_invite.as_keyset_root_authority_ref().clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_keyset_root_authority_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(Some(keyset_root_authority_element.clone())));

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_parent_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(None));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![device_invite.as_parent_ref().clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_keyset_root_authority_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(Some(keyset_root_authority_element.clone())));

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_parent_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(2)
            .return_const(Ok(Some(parent_element.clone())));

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        parent.as_invite_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(None));

        mock_hdk.expect_zome_info()
            .times(1)
            .return_const(Ok(zome_info.clone()));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::UnresolvedDependencies(vec![parent.as_invite_ref().clone().into()])),
        );

        let mut mock_hdk = MockHdkT::new();

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_keyset_root_authority_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(Some(keyset_root_authority_element.clone())));

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        device_invite.as_parent_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(2)
            .return_const(Ok(Some(parent_element.clone())));

        mock_hdk.expect_get()
            .with(
                mockall::predicate::eq(
                    GetInput::new(
                        parent.as_invite_ref().clone().into(),
                        GetOptions::content(),
                    )
                )
            )
            .times(1)
            .return_const(Ok(Some(parent_invite_element.clone())));

        mock_hdk.expect_zome_info().times(1).return_const(Ok(zome_info));

        set_hdk(mock_hdk);

        assert_eq!(
            super::validate_create_entry_device_invite(validate_data.clone()),
            Ok(ValidateCallbackResult::Valid),
        );
    }
}
