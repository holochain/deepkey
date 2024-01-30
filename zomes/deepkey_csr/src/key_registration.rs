use crate::utils;
use deepkey::*;
use hdk::prelude::{
    *,
    holo_hash::DnaHash,
};
use hdk_extensions::{
    agent_id,
    must_get,
    hdi_extensions::{
        guest_error,
        trace_origin_root,
        ScopedTypeConnector,
    },
};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppBindingInput {
    pub app_name: String,
    pub dna_hashes: Vec<DnaHash>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DerivationDetailsInput {
    pub app_index: u32,
    pub key_index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateKeyInput {
    pub key_generation: KeyGeneration,
    pub app_binding: AppBindingInput,
    pub derivation_details: DerivationDetailsInput,
}

#[hdk_extern]
pub fn create_key(input: CreateKeyInput) -> ExternResult<(ActionHash, KeyRegistration, KeyMeta)> {
    let key_gen = input.key_generation;
    let given_app_index = input.derivation_details.app_index;
    let given_key_index = input.derivation_details.key_index;
    let chain_app_index = crate::app_binding::query_app_bindings(())?.len() as u32;

    // Check that derivation details match the chain state
    if given_app_index != chain_app_index {
        Err(guest_error!(format!(
            "The derivation app index does not match the chain state: [given] {} != {} [chain]",
            given_app_index, chain_app_index,
        )))?
    }
    if given_key_index != 0 {
        Err(guest_error!(format!(
            "The derivation key index must be 0 for a new key registration: [given] {} != 0",
            given_key_index,
        )))?
    }

    // Derive Key Anchor
    let key_anchor = KeyAnchor::try_from( &key_gen.new_key )?;
    let key_anchor_hash = hash_entry( &key_anchor.to_input() )?;

    // Create Registration
    let key_reg = KeyRegistration::Create(
        (
            &key_gen.new_key,
            &key_gen.new_key_signing_of_author
        ).into()
    );
    let key_reg_addr = create_entry( key_reg.to_input() )?;

    // Create Anchor
    let key_anchor_addr = create_entry( key_anchor.to_input() )?;

    // Create App Binding
    let app_binding = AppBinding {
        app_index: given_app_index,
        app_name: input.app_binding.app_name,
        dna_hashes: input.app_binding.dna_hashes,
        key_anchor_addr: key_anchor_addr.clone(),
    };
    let app_binding_addr = create_entry( app_binding.to_input() )?;

    // Create Meta
    let key_meta = KeyMeta {
        app_binding_addr: app_binding_addr.clone(),
        key_index: 0,
        key_registration_addr: key_reg_addr.clone(),
        key_anchor_addr: key_anchor_addr.clone(),
    };
    let key_meta_addr = create_entry( key_meta.to_input() )?;

    // Link Binding -> Meta
    create_link(
        app_binding_addr.clone(),
        key_meta_addr.clone(),
        LinkTypes::AppBindingToKeyMeta,
        (),
    )?;

    // Link Device -> Registration::Create
    let device_base = agent_id()?;
    create_link(
        device_base,
        key_anchor_hash,
        LinkTypes::DeviceToKeyAnchor,
        (),
    )?;

    Ok((
        key_reg_addr,
        key_reg,
        key_meta,
    ))
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateKeyInput {
    pub key_revocation: KeyRevocation,
    pub key_generation: KeyGeneration,
    pub derivation_details: DerivationDetailsInput,
}

#[hdk_extern]
pub fn update_key(input: UpdateKeyInput) -> ExternResult<(ActionHash, KeyRegistration, KeyMeta)> {
    let key_rev = input.key_revocation;
    let key_gen = input.key_generation;

    let given_app_index = input.derivation_details.app_index;
    let given_key_index = input.derivation_details.key_index;

    let (app_binding_addr, _) = crate::app_binding::query_app_binding_for_index( given_app_index )?;

    let state_key_index = crate::key_meta::query_key_metas_for_app_binding(
        app_binding_addr.clone()
    )?.len() as u32;

    // Check that prior key meta has the same app index
    let prior_key_meta = crate::key_meta::query_key_meta_for_registration(
        key_rev.prior_key_registration.clone()
    )?;

    if prior_key_meta.app_binding_addr != app_binding_addr {
        Err(guest_error!(format!(
            "Prior app binding ({}) does not match the app binding ({}) found for index: {}",
            prior_key_meta.app_binding_addr, app_binding_addr, given_app_index,
        )))?
    }

    if given_key_index != state_key_index {
        Err(guest_error!(format!(
            "The derivation key index does not match the key state: [given] {} != {} [state]",
            given_key_index, state_key_index
        )))?
    }

    // Derive Key Anchor
    let key_anchor = KeyAnchor::try_from( &key_gen.new_key )?;
    let key_anchor_hash = hash_entry( &key_anchor.to_input() )?;

    // Create Registration
    let prior_key_registration = key_rev.prior_key_registration.clone();
    let key_reg = KeyRegistration::Update( key_rev, key_gen );
    let key_reg_addr = update_entry( prior_key_registration, key_reg.to_input() )?;

    // Create Anchor
    let key_anchor_addr = create_entry( key_anchor.to_input() )?;

    // Create Meta
    let key_meta = KeyMeta {
        app_binding_addr: app_binding_addr.clone(),
        key_index: 0,
        key_registration_addr: key_reg_addr.clone(),
        key_anchor_addr: key_anchor_addr.clone(),
    };
    let key_meta_addr = create_entry( key_meta.to_input() )?;

    // Link Binding -> Meta
    create_link(
        app_binding_addr.clone(),
        key_meta_addr.clone(),
        LinkTypes::AppBindingToKeyMeta,
        (),
    )?;

    // Link Device -> Registration
    let device_base = agent_id()?;
    create_link(
        device_base,
        key_anchor_hash,
        LinkTypes::DeviceToKeyAnchor,
        (),
    )?;

    Ok((
        key_reg_addr,
        key_reg,
        key_meta,
    ))
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevokeKeyInput {
    pub key_revocation: KeyRevocation,
}

#[hdk_extern]
pub fn revoke_key(input: RevokeKeyInput) -> ExternResult<(ActionHash, KeyRegistration)> {
    let key_rev = input.key_revocation;

    // Get key anchor create action
    let prior_key_addr = crate::key_anchor::get_key_anchor_for_registration(
        key_rev.prior_key_registration.clone()
    )?.0;
    let key_anchor_create_addr = trace_origin_root( &prior_key_addr )?.0;
    delete_entry( key_anchor_create_addr )?;

    let key_revocation = KeyRevocation {
        prior_key_registration: key_rev.prior_key_registration.clone(),
        revocation_authorization: key_rev.revocation_authorization,
    };
    let revocation_registration = KeyRegistration::Delete(key_revocation);

    // TODO: Fill out the validation for KeyRevocation so it actually validates the revocation_authorization signatures.
    let key_reg_addr = update_entry(
        key_rev.prior_key_registration,
        revocation_registration.to_input(),
    )?;

    Ok((
        key_reg_addr,
        revocation_registration,
    ))
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
