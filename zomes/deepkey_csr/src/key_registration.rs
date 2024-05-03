use crate::{
    utils,
};
use serde_bytes::ByteArray;
use deepkey::*;
use hdk::prelude::*;
use hdk_extensions::{
    must_get,
    hdi_extensions::{
        guest_error,
        ScopedTypeConnector,
    },
};
pub use deepkey_sdk::{
    AppBindingInput,
    CreateKeyInput,
    UpdateKeyInput,
    RevokeKeyInput,
    KeyRevocationInput,
    DerivationDetails,
    DerivationDetailsInput,
};


#[hdk_extern]
pub fn next_derivation_details(
    input: Option<ByteArray<32>>
) -> ExternResult<DerivationDetails> {
    Ok(
        match input {
            Some(key_bytes) => {
                let (_, app_binding) = crate::app_binding::query_app_binding_by_key( key_bytes )?;

                let next_key_index = crate::key_meta::query_next_key_index_for_app_index(
                    app_binding.app_index
                )?;

                DerivationDetails {
                    app_index: app_binding.app_index,
                    key_index: next_key_index,
                }
            },
            None => {
                DerivationDetails {
                    app_index: crate::app_binding::query_next_app_index(())?,
                    key_index: 0,
                }
            },
        }
    )
}


#[hdk_extern]
pub fn get_key_derivation_details(
    key_bytes: ByteArray<32>
) -> ExternResult<DerivationDetails> {
    let key_meta = crate::key_meta::query_key_meta_for_key( key_bytes.clone() )?;
    let (_, app_binding) = crate::app_binding::query_app_binding_by_key( key_bytes.clone() )?;

    Ok(
        DerivationDetails {
            app_index: app_binding.app_index,
            key_index: key_meta.key_index,
        }
    )
}


#[hdk_extern]
pub fn create_key(input: CreateKeyInput) -> ExternResult<(ActionHash, KeyRegistration, KeyMeta)> {
    let key_gen = input.key_generation;
    let next_app_index = crate::app_binding::query_next_app_index(())?;

    // Check that derivation details match the chain state
    if let Some(derivation_details) = &input.derivation_details {
        let given_app_index = derivation_details.app_index;
        let given_key_index = derivation_details.key_index;

        if given_app_index != next_app_index {
            Err(guest_error!(format!(
                "The derivation app index does not match the chain state: [given] {} != {} [next]",
                given_app_index, next_app_index,
            )))?
        }
        if given_key_index != 0 {
            Err(guest_error!(format!(
                "The derivation key index must be 0 for a new key registration: [given] {} != 0",
                given_key_index,
            )))?
        }
    }

    // Derive Key Anchor
    let key_anchor = KeyAnchor::try_from( &key_gen.new_key )?;

    // Create Registration
    let key_gen = KeyGeneration::from((
        &key_gen.new_key,
        &key_gen.new_key_signing_of_author
    ));
    let key_reg = match input.create_only {
        true => KeyRegistration::CreateOnly(key_gen),
        false => KeyRegistration::Create(key_gen),
    };
    let key_reg_addr = create_entry( key_reg.to_input() )?;

    // Create Anchor
    let key_anchor_addr = crate::key_anchor::create_key_anchor( key_anchor )?;

    // Create App Binding
    let app_binding = AppBinding {
        app_index: next_app_index,
        app_name: input.app_binding.app_name,
        installed_app_id: input.app_binding.installed_app_id,
        dna_hashes: input.app_binding.dna_hashes,
        key_anchor_addr: key_anchor_addr.clone(),
        metadata: input.app_binding.metadata,
    };
    let app_binding_addr = create_entry( app_binding.to_input() )?;

    // Create Meta
    let key_meta = KeyMeta {
        app_binding_addr: app_binding_addr.clone(),
        key_index: 0,
        key_registration_addr: key_reg_addr.clone(),
        key_anchor_addr: key_anchor_addr.clone(),
        derivation_seed: input.derivation_details.as_ref()
            .map( |details| details.derivation_seed.to_owned() ),
        derivation_bytes: input.derivation_details.as_ref()
            .map( |details| details.derivation_bytes.to_owned() ),
    };
    create_entry( key_meta.to_input() )?;

    Ok((
        key_reg_addr,
        key_reg,
        key_meta,
    ))
}


#[hdk_extern]
pub fn update_key(input: UpdateKeyInput) -> ExternResult<(ActionHash, KeyRegistration, KeyMeta)> {
    let key_rev = input.key_revocation;
    let key_gen = input.key_generation;
    let prior_key_reg_addr = key_rev.prior_key_registration.clone();

    let prior_key_meta = crate::key_meta::query_key_meta_for_registration(
        prior_key_reg_addr.clone()
    )?;
    let app_binding = crate::app_binding::query_app_binding_by_action(
        prior_key_meta.app_binding_addr.clone()
    )?;
    let next_key_index = crate::key_meta::query_next_key_index_for_app_index( app_binding.app_index )?;

    // Check that derivation details match the chain state
    if let Some(derivation_details) = &input.derivation_details {
        let given_app_index = derivation_details.app_index;
        let given_key_index = derivation_details.key_index;

        // Check that derivation details has the correct 'app_index'
        if given_app_index != app_binding.app_index {
            Err(guest_error!(format!(
                "The derivation app index does not match the app binding: [given] {} != {} [prior]",
                given_app_index, app_binding.app_index,
            )))?
        }

        // Check that derivation details has the correct 'key_index'
        if given_key_index != next_key_index {
            Err(guest_error!(format!(
                "The derivation key index does not match the chain state: [given] {} != {} [next]",
                given_key_index, next_key_index
            )))?
        }
    }

    // Derive Key Anchor
    let key_anchor = KeyAnchor::try_from( &key_gen.new_key )?;

    // Create Registration
    let key_reg = KeyRegistration::Update( key_rev, key_gen );
    let key_reg_addr = update_entry( prior_key_reg_addr.clone(), key_reg.to_input() )?;

    // Create Anchor
    let prior_key_addr = crate::key_anchor::get_key_anchor_for_registration(
        prior_key_reg_addr.clone()
    )?.0;
    let key_anchor_addr = update_entry( prior_key_addr, key_anchor.to_input() )?;

    // Create Meta
    let key_meta = KeyMeta {
        app_binding_addr: prior_key_meta.app_binding_addr.clone(),
        key_index: next_key_index,
        key_registration_addr: key_reg_addr.clone(),
        key_anchor_addr: key_anchor_addr.clone(),
        derivation_seed: input.derivation_details.as_ref()
            .map( |details| details.derivation_seed.to_owned() ),
        derivation_bytes: input.derivation_details.as_ref()
            .map( |details| details.derivation_bytes.to_owned() ),
    };
    create_entry( key_meta.to_input() )?;

    Ok((
        key_reg_addr,
        key_reg,
        key_meta,
    ))
}


#[hdk_extern]
pub fn revoke_key(input: RevokeKeyInput) -> ExternResult<(ActionHash, KeyRegistration)> {
    let key_rev = input.key_revocation;
    let prior_key_reg_addr = key_rev.prior_key_registration.clone();

    let key_revocation = KeyRevocation {
        prior_key_registration: prior_key_reg_addr.clone(),
        revocation_authorization: key_rev.revocation_authorization,
    };
    let registration_delete = KeyRegistration::Delete(key_revocation);

    let key_reg_addr = update_entry(
        prior_key_reg_addr.clone(),
        registration_delete.to_input(),
    )?;

    // Terminate key anchor
    let prior_key_addr = crate::key_anchor::get_key_anchor_for_registration(
        prior_key_reg_addr.clone()
    )?.0;
    delete_entry( prior_key_addr )?;

    Ok((
        key_reg_addr,
        registration_delete,
    ))
}


#[hdk_extern]
pub fn delete_key_registration(
    input: (ActionHash, KeyRevocationInput),
) -> ExternResult<ActionHash> {
    update_entry(
        input.0,
        KeyRegistration::Delete( KeyRevocation::try_from( input.1 )? ).to_input(),
    )
}


#[hdk_extern]
pub fn get_key_registration(addr: ActionHash) -> ExternResult<KeyRegistration> {
    must_get( &addr )?.try_into()
}


#[hdk_extern]
fn get_latest_key_registration(addr: ActionHash) -> ExternResult<(ActionHash, KeyRegistration)> {
    let record = utils::get_latest_record( addr )?;

    Ok((
        record.action_address().to_owned(),
        record.try_into()?,
    ))
}
