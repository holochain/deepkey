use crate::utils;
use crate::hdi_extensions::{
    guest_error,
    ScopedTypeConnector,
};
use crate::hdk_extensions::{
    agent_id,
    must_get,
};

use deepkey::*;
use hdk::prelude::*;


#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<ActionHash> {
    // let membrane_proof = utils::my_membrane_proof()?;
    // debug!("Membrane proof: {:?}", membrane_proof );

    let fda: AgentPubKey = agent_info()?.agent_latest_pubkey;
    let fda_bytes = fda.clone().into_inner();

    let esigs = sign_ephemeral_raw(vec![ fda_bytes ])?;
    let [signed_fda, ..] = esigs.signatures.as_slice() else {
        return Err(guest_error!("sign_ephemeral returned wrong number of signatures".to_string()))
    };

    let keyset_root = KeysetRoot::new(
        fda.clone(),
        esigs.key.get_raw_32().try_into()
            .map_err( |e| guest_error!(format!("Failed AgentPubKey to [u8;32] conversion: {:?}", e)) )?,
        signed_fda.to_owned()
    );
    let create_hash = create_entry( keyset_root.to_input() )?;

    init_change_rule( 1, vec![
        fda.get_raw_32().try_into()
            .map_err(|e| guest_error!(format!(
                "FDA.get_raw_32() did not have 32 elements; this should be unreachable -> {}", e
            )))?,
    ])?;

    // Register the FDA as a key under this KSR
    use crate::key_registration::{
        CreateKeyInput,
        AppBindingInput,
        DerivationDetailsInput,
    };

    let dna_hash = dna_info()?.hash;

    crate::key_registration::create_key(CreateKeyInput {
        key_generation: KeyGeneration {
            new_key: fda.clone(),
            new_key_signing_of_author: sign_raw( fda, agent_id()?.into_inner() )?,
        },
        app_binding: AppBindingInput {
            app_name: "deepkey".to_string(),
            installed_app_id: "deepkey".to_string(),
            dna_hashes: vec![ dna_hash ],
            metadata: Default::default(),
        },
        derivation_details: Some(DerivationDetailsInput {
            app_index: 0,
            key_index: 0,
            derivation_seed: vec![],
            derivation_bytes: vec![],
        }),
    })?;

    Ok( create_hash )
}


pub fn init_change_rule(
    sigs_required: u8,
    revocation_keys: Vec<KeyBytes>
) -> ExternResult<ActionHash> {
    let ksr_addr = utils::query_keyset_root_addr()?;
    let new_authority_spec = AuthoritySpec::new(
        sigs_required,
        revocation_keys,
    );
    let auth_spec_bytes = utils::serialize( &new_authority_spec )?;
    let signed_bytes = sign( agent_id()?, auth_spec_bytes )?;

    let spec_change = AuthorizedSpecChange::new(
        new_authority_spec, vec![(0, signed_bytes)]
    );

    let change_rule = ChangeRule::new(
        ksr_addr.clone(),
        ksr_addr.clone(),
        spec_change,
    );

    Ok( crate::change_rule::create_change_rule( change_rule )? )
}


#[hdk_extern]
pub fn get_keyset_root(ksr_addr: ActionHash) -> ExternResult<KeysetRoot> {
    must_get( &ksr_addr )?.try_into()
}


// Get all of the keys registered on the keyset, across all the deepkey agents
#[hdk_extern]
pub fn query_apps_with_keys(_:()) -> ExternResult<Vec<(AppBinding, Vec<KeyMeta>)>> {
    let key_metas : Vec<(ActionHash, KeyMeta)> = utils::query_entry_type( EntryTypesUnit::KeyMeta )?
        .into_iter()
        .filter_map( |record| Some((
            record.action_address().to_owned(),
            KeyMeta::try_from(record).ok()?,
        )))
        .collect();

    Ok(
        utils::query_entry_type( EntryTypesUnit::AppBinding )?
            .into_iter()
            .filter( |record| record.action().action_type() == ActionType::Create )
            .filter_map( |record| Some((
                record.action_address().to_owned(),
                AppBinding::try_from(record).ok()?,
            )))
            .map( |(addr, app_binding)| {
                (
                    app_binding,
                    key_metas.iter()
                        .filter( |(_, key_meta)| key_meta.app_binding_addr == addr )
                        .map( |(_, key_meta)| key_meta )
                        .cloned()
                        .collect(),
                )
            })
            .collect()
    )
}


// #[hdk_extern]
// pub fn query_key_registrations(_:()) -> ExternResult<Vec<(ActionHash, Vec<KeyRegistration>)>> {
// }
