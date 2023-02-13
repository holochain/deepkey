use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<(Record, Record)> {
    let first_deepkey_agent: AgentPubKey = agent_info()?.agent_latest_pubkey;

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

    // TODO: Should we make JoiningProof its own struct, or use holochain's joining proof functionality directly?
    // let joining_proof = JoiningProof::new(
    //     MembraneProof::None,
    // );
    // let joining_proof_hash = create_entry(EntryTypes::JoiningProof(joining_proof))?;
    let keyset_root_hash = create_entry(EntryTypes::KeysetRoot(KeysetRoot::new(
        first_deepkey_agent.clone(),
        root_pub_key,
        fda_signature,
    )))?;

    let spec_change = AuthorizedSpecChange::new(new_authority_spec, vec![(0, auth_spec_signature)]);
    let change_rule_hash = create_entry(EntryTypes::ChangeRule(ChangeRule::new(
        keyset_root_hash.clone(),
        keyset_root_hash.clone(),
        spec_change,
    )))?;

    let keyset_root_record =
        get(keyset_root_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the newly created Keyset Root"))
        ))?;

    let change_rule_record =
        get(change_rule_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the newly created Change Rule"))
        ))?;

    Ok((keyset_root_record, change_rule_record))
}
#[hdk_extern]
pub fn get_keyset_root(keyset_root_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(keyset_root_hash, GetOptions::default())
}
