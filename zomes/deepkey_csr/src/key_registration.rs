use crate::{
    utils,
    deepkey_sdk,
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
    key_bytes: ByteArray<32>
) -> ExternResult<DerivationDetails> {
    let (_, app_binding) = crate::app_binding::query_app_binding_by_key( key_bytes )?;

    let next_key_index = crate::key_meta::query_next_key_index_for_app_index(
        app_binding.app_index
    )?;

    Ok(DerivationDetails {
        app_index: app_binding.app_index,
        key_index: next_key_index,
    })
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
pub fn check_existing_derivation_details(
    derivation_details: DerivationDetailsInput,
) -> ExternResult<Option<(AppBinding, KeyMeta)>> {
    for key_meta in crate::key_meta::query_key_metas(())? {
        if let Some(ref derivation_bytes) = key_meta.derivation_bytes {
            if *derivation_bytes == derivation_details.derivation_bytes {
                let app_binding = crate::app_binding::query_app_binding_by_action(
                    key_meta.app_binding_addr.to_owned()
                )?;

                return Ok(Some((app_binding, key_meta)));
            }
        }

        let app_binding = crate::app_binding::query_app_binding_by_action(
            key_meta.app_binding_addr.to_owned()
        )?;

        if derivation_details.app_index == app_binding.app_index &&
            Some(derivation_details.derivation_seed.to_owned()) == key_meta.derivation_seed {
                return Ok(Some((app_binding, key_meta)));
        }
    }

    Ok(None)
}


/// Register a new app/key pair and create associated private entries
///
/// #### Example usage
/// ```rust, no_run
/// # use hdk::prelude::*;
/// # use deepkey::*;
/// # use hc_deepkey_sdk::*;
/// # fn main() -> ExternResult<()> {
/// // Generates a new key in Lair
/// let new_key = AgentPubKey::from_raw_32( create_x25519_keypair()?.as_ref().to_vec() );
/// // Sign this cell's agent with using the new key
/// let new_key_signing_of_author = sign_raw(
///     new_key.clone(),
///     agent_info()?.agent_initial_pubkey.into_inner()
/// )?;
///
/// let key_generation = KeyGeneration {
///     new_key,
///     new_key_signing_of_author,
/// };
///
/// // Mock app info
/// let app_binding = AppBindingInput {
///     app_name: "Example App".to_string(),
///     installed_app_id: "example-app".to_string(),
///     dna_hashes: vec![
///         DnaHash::try_from("uhC0khcxkMswniVr_dwGJAo2spTGC-hafG0lCEvzS_PJughwa4_6d").unwrap(),
///     ],
///     metadata: Default::default(),
/// };
///
/// // Here is an example of submitting the derivation details; however, in this case it should
/// // be set to `None` because we did not derive this key.
/// let derivation_details = Some(DerivationDetailsInput {
///     app_index: 1, // 0 is already used by the deepkey app
///     key_index: 0,
///     derivation_seed: vec![],
///     derivation_bytes: vec![],
/// });
///
/// let result = deepkey_csr::key_registration::create_key(CreateKeyInput {
///     key_generation,
///     app_binding,
///     derivation_details,
///     create_only: false,
/// });
/// # Ok(())
/// # }
/// ```
#[hdk_extern]
pub fn create_key(input: CreateKeyInput) -> ExternResult<(ActionHash, KeyRegistration, KeyMeta)> {
    let key_gen = input.key_generation;
    let next_app_index = crate::app_binding::query_next_app_index(())?;

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


/// Register a key update for an existing app/key pair and create associated private entries
///
/// #### Example usage
/// ```rust, no_run
/// # use hdk::prelude::*;
/// # use deepkey::*;
/// # use hc_deepkey_sdk::*;
/// # fn main() -> ExternResult<()> {
/// let prior_key_registration = ActionHash::try_from("uhCkkzhwfnkYh7CWji2KpS2wO6YaKOKPQ4-kr4XGRBRRx9hitvOw9").unwrap();
///
/// // Assuming the default change rules are still in place
/// let revocation_authorization = vec![
///     (
///         0, // The index of the FDA authority
///         sign_raw( // Sign prior registration using FDA
///             agent_info()?.agent_initial_pubkey,
///             prior_key_registration.clone().into_inner()
///         )?
///     ),
/// ];
///
/// let key_revocation = KeyRevocation {
///     prior_key_registration,
///     revocation_authorization,
/// };
///
/// // Generates a new key in Lair
/// let new_key = AgentPubKey::from_raw_32( create_x25519_keypair()?.as_ref().to_vec() );
/// // Sign this cell's agent with using the new key
/// let new_key_signing_of_author = sign_raw(
///     new_key.clone(),
///     agent_info()?.agent_initial_pubkey.into_inner()
/// )?;
///
/// let key_generation = KeyGeneration {
///     new_key,
///     new_key_signing_of_author,
/// };
///
/// // Here is an example of submitting the derivation details; however, in this case it should
/// // be set to `None` because we did not derive this key.
/// let derivation_details = Some(DerivationDetailsInput {
///     app_index: 1,
///     key_index: 1, // Next key index
///     derivation_seed: vec![],
///     derivation_bytes: vec![],
/// });
///
/// let result = deepkey_csr::key_registration::update_key(UpdateKeyInput {
///     key_revocation,
///     key_generation,
///     derivation_details,
/// });
/// # Ok(())
/// # }
/// ```
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

        // Check that derivation details has the correct 'app_index'
        if given_app_index != app_binding.app_index {
            Err(guest_error!(format!(
                "The derivation app index does not match the app binding: [given] {} != {} [prior]",
                given_app_index, app_binding.app_index,
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
        key_index: input.derivation_details.as_ref()
            .map( |details| details.key_index.to_owned() )
            .unwrap_or( next_key_index ),
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


/// Register a key delete for an existing app/key pair
///
/// #### Example usage
/// ```rust, no_run
/// # use hdk::prelude::*;
/// # use deepkey::*;
/// # use hc_deepkey_sdk::*;
/// # fn main() -> ExternResult<()> {
/// let prior_key_registration = ActionHash::try_from("uhCkkzhwfnkYh7CWji2KpS2wO6YaKOKPQ4-kr4XGRBRRx9hitvOw9").unwrap();
///
/// // Assuming the default change rules are still in place
/// let revocation_authorization = vec![
///     (
///         0, // The index of the FDA authority
///         sign_raw( // Sign prior registration using FDA
///             agent_info()?.agent_initial_pubkey,
///             prior_key_registration.clone().into_inner()
///         )?
///     ),
/// ];
///
/// let key_revocation = KeyRevocation {
///     prior_key_registration,
///     revocation_authorization,
/// };
///
/// let result = deepkey_csr::key_registration::revoke_key(RevokeKeyInput {
///     key_revocation,
/// });
/// # Ok(())
/// # }
/// ```
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
