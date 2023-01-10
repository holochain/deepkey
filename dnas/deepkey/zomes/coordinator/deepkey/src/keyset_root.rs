use hdk::prelude::*;

use deepkey_integrity::{
    change_rule::{AuthoritySpec, AuthorizedSpecChange, ChangeRule},
    keyset_root::KeysetRoot,
    EntryTypes, JoiningProof, KeysetProof, MembraneProof,
};

#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<(ActionHash, ActionHash)> {
    let first_deepkey_agent: AgentPubKey = agent_info()?.agent_latest_pubkey;

    // TODO: Make sure this is the first JoiningProof

    // There is only one authorized signer: the first deepkey agent (fda)
    let new_authority_spec = AuthoritySpec::new(1, vec![first_deepkey_agent.clone()]);

    let fda_bytes_result: Result<SerializedBytes, _> = first_deepkey_agent.clone().try_into();
    let fda_bytes = fda_bytes_result.map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    let new_authority_spec_bytes_result: Result<SerializedBytes, _> =
        new_authority_spec.clone().try_into();
    let new_authority_spec_bytes = new_authority_spec_bytes_result
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;

    let sigs = sign_ephemeral::<SerializedBytes>(vec![fda_bytes, new_authority_spec_bytes])?;
    let root_pub_key = sigs.key;
    let mut sig_iter = sigs.signatures.into_iter();
    let sig_error_closure = || {
        wasm_error!(WasmErrorInner::Guest(
            "Expected an ephemeral signature".into()
        ))
    };

    let fda_signature = sig_iter.next().ok_or_else(sig_error_closure)?;
    let auth_spec_signature = sig_iter.next().ok_or_else(sig_error_closure)?;

    let joining_proof = JoiningProof::new(
        KeysetProof::KeysetRoot(KeysetRoot::new(
            first_deepkey_agent.clone(),
            root_pub_key,
            fda_signature,
        )),
        MembraneProof::None,
    );
    let keyset_root_hash = create_entry(EntryTypes::JoiningProof(joining_proof))?;

    let spec_change = AuthorizedSpecChange::new(new_authority_spec, vec![(0, auth_spec_signature)]);
    // TODO: keyset_leaf can be a device invite acceptance here, depending on the keyset proof type
    let change_rule = ChangeRule::new(
        keyset_root_hash.clone(),
        keyset_root_hash.clone(),
        spec_change,
    );
    let change_rule_hash = create_entry(EntryTypes::ChangeRule(change_rule))?;

    Ok((keyset_root_hash, change_rule_hash))
}
