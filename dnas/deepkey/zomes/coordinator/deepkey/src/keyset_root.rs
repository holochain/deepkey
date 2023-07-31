use deepkey_integrity::*;
use hdk::prelude::*;

#[hdk_extern]
pub fn create_keyset_root(_: ()) -> ExternResult<(ActionHash, ActionHash)> {
    let first_deepkey_agent: AgentPubKey = agent_info()?.agent_latest_pubkey;

    // There is only one authorized signer: the first deepkey agent (fda)
    let new_authority_spec = AuthoritySpec::new(0, vec![first_deepkey_agent.clone()]);

    let fda_bytes = SerializedBytes::try_from(first_deepkey_agent.clone())
        .map_err(|e| wasm_error!(WasmErrorInner::Guest(e.into())))?;
    let new_authority_spec_bytes = SerializedBytes::try_from(new_authority_spec.clone())
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

    let keyset_root = KeysetRoot::new(first_deepkey_agent.clone(), root_pub_key, fda_signature);
    let keyset_root_hash = create_entry(EntryTypes::KeysetRoot(keyset_root))?;

    let spec_change = AuthorizedSpecChange::new(new_authority_spec, vec![(0, auth_spec_signature)]);
    // TODO: Should the keyset_leaf here be a SourceOfAuthority::KeysetRoot hash?
    let change_rule_hash = create_entry(EntryTypes::ChangeRule(ChangeRule::new(
        keyset_root_hash.clone(),
        keyset_root_hash.clone(),
        spec_change,
    )))?;

    // let keyset_root_record =
    //     get(keyset_root_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
    //         WasmErrorInner::Guest(String::from("Could not find the newly created Keyset Root"))
    //     ))?;

    // let change_rule_record =
    //     get(change_rule_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
    //         WasmErrorInner::Guest(String::from("Could not find the newly created Change Rule"))
    //     ))?;
    // Ok((keyset_root_record, change_rule_record))

    // Err(wasm_error!(WasmErrorInner::Guest(String::from(
    //     "Testing an error message here"
    // ))))
    Ok((keyset_root_hash, change_rule_hash))
}

#[hdk_extern]
pub fn get_keyset_root(keyset_root_hash: ActionHash) -> ExternResult<Option<Record>> {
    get(keyset_root_hash, GetOptions::default())
}

#[hdk_extern]
pub fn query_keyset_members(keyset_root_hash: ActionHash) -> ExternResult<Vec<AgentPubKey>> {
    // Get the PubKey of the Deepkey Agent who wrote the KeysetRoot
    let keyset_root_record =
        get(keyset_root_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the Keyset Root"))
        ))?;
    let ksr_chain_pubkey = keyset_root_record.action().author().clone();

    let dia_hashes: Vec<ActionHash> = get_links(
        keyset_root_hash,
        LinkTypes::KeysetRootToDeviceInviteAcceptances,
        None,
    )?
    .into_iter()
    .map(|link| link.target.into())
    .collect();
    let dia_records: Vec<Record> = dia_hashes
        .into_iter()
        .map(|dia_hash| get(dia_hash, GetOptions::default()))
        .collect::<ExternResult<Vec<Option<Record>>>>()?
        .into_iter()
        .filter_map(|x| x)
        .collect();

    // Query all the Deepkey Agents that wrote the DeviceInviteAcceptances
    let mut dia_author_pubkeys = dia_records
        .into_iter()
        .map(|dia_record| dia_record.action().author().clone())
        .collect::<Vec<AgentPubKey>>();

    // Don't forget the First Deepkey Agent
    dia_author_pubkeys.push(ksr_chain_pubkey);

    Ok(dia_author_pubkeys)
}

#[hdk_extern]
pub fn query_keyset_keys(keyset_root_hash: ActionHash) -> ExternResult<Vec<KeyAnchor>> {
    let keyset_root_record = get(keyset_root_hash, GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the Keyset Root"))
    ))?;
    // go from ksr, to list of all key anchors registered on this keyset

    Err(wasm_error!(WasmErrorInner::Guest("Not implemented".into())))
}
