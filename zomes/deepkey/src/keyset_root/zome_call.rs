use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::keyset_root::entry::KeysetRoot;
use deepkey_integrity::change_rule::entry::{ AuthorizedSpecChange, AuthoritySpec, Authorization, ChangeRule };
use deepkey_integrity::entry::EntryTypes;

///
/// generate_keyset_root
/// 
///     Creates a new KeysetRoot for the specified First Deepkey Agent (or, this Agent), signed
/// by an ephemeral key.  The 32-byte Agent PubKey's Signature is used.
/// 
#[hdk_extern]
fn generate_keyset_root((first_deepkey_agent,): (Option<AgentPubKey>,)) -> ExternResult<KeysetRoot> {
    let first_deepkey_agent = first_deepkey_agent.unwrap_or(agent_info()?.agent_initial_pubkey);
    debug!("generate_keyset_root for {:?}", first_deepkey_agent);
    let EphemeralSignatures{ key: root_pub_key, signatures } = sign_ephemeral_raw(vec![
        first_deepkey_agent.clone().get_raw_32().to_vec()]
    )?;
    let fda_pubkey_signed_by_root_key = signatures[0].clone();
    let keyset_root = KeysetRoot{ first_deepkey_agent, root_pub_key, fda_pubkey_signed_by_root_key };
    debug!("generate_keyset_root: {:?}", keyset_root);
    Ok(keyset_root)
}
    
/// 
/// create_keyset_root -- Create an initial KeysetRoot, or switch to a new KeysetRoot w/ proper authority
/// 
///     When initializing a Deepkey, the ChangeRule{ keyset_root, keyset_leaf, ... } will both
/// reference the KeysetRoot just committed.  Otherwise, the provided keyset_root indicates that we
/// are switching from an old KeysetRoot to a new one (just committed) at ActionHash keyset_leaf.
/// 
///     If the validation for the ChangeRule fails (improper authority provided), the commit of the
/// new KeysetRoot is rolled back.
/// 
#[hdk_extern]
fn create_keyset_root((new_keyset_root, spec_change, keyset_root): (KeysetRoot, Option<AuthorizedSpecChange>, Option<ActionHash>)) -> ExternResult<(ActionHash, ActionHash)> {
    debug!("create_keyset_root {:?}, {:?}, {:?}", new_keyset_root, spec_change, keyset_root);
    let keyset_leaf = create_entry(EntryTypes::KeysetRoot(new_keyset_root))?;
    // Default to the simplest AuthorizedSpecChange: just this Agent.
    let agent = agent_info()?.agent_initial_pubkey;
    let spec_change = spec_change.unwrap_or({
        let new_spec = AuthoritySpec{ sigs_required: 1, authorized_signers: vec![agent.clone()] };
        let authorization_of_new_spec: Vec<Authorization> = vec![
            (0, sign( agent, new_spec.clone() )?)
        ];
        AuthorizedSpecChange{ new_spec, authorization_of_new_spec }
    });
    let change_rule = create_entry(EntryTypes::ChangeRule(match keyset_root {
        // If we're initializing Deepkey w/ its first KeysetRoot
        None => ChangeRule{
            keyset_leaf: keyset_leaf.clone(), keyset_root: keyset_leaf.clone(), spec_change
        },
        // If we're updating an existing keyset_root with a new KeysetRoot
        Some(keyset_root) => ChangeRule{
            keyset_leaf: keyset_leaf.clone(), keyset_root, spec_change
        },
    }))?;
    Ok((keyset_leaf, change_rule))
}
