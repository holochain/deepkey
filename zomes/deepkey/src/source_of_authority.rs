use crate::*;
use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub enum SourceOfAuthority {
    KeysetRoot(KeysetRoot),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_extern]
pub fn get_remote_source_of_authority_action_hash(
    (agent, from_action_hash): (AgentPubKey, ActionHash),
) -> ExternResult<ActionHash> {
    must_get_agent_activity(agent, ChainFilter::new(from_action_hash))?
        .into_iter()
        .find_map(|activity| {
            if let Some(EntryType::App(app_entry_def)) = activity.action.action().entry_type() {
                if *app_entry_def == AppEntryDef::try_from(EntryTypesUnit::KeysetRoot).unwrap()
                    || *app_entry_def
                        == AppEntryDef::try_from(EntryTypesUnit::DeviceInviteAcceptance).unwrap()
                {
                    return Some(activity.action.as_hash().clone());
                }
            };
            None
        })
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from(
            "Could not find the source of authority: Deepkey agent has no Keyset Root!"
        ))))
}


/// Takes the action hash of a KeysetRoot or DeviceInviteAcceptance and returns the corresponding
/// SourceOfAuthority
pub fn hydrate_source_of_authority(soa_action_hash: ActionHash) -> ExternResult<SourceOfAuthority> {
    must_get_action(soa_action_hash).and_then(|action| {
        if let Some((entry_hash, EntryType::App(app_entry_def))) = action.action().entry_data() {
            if *app_entry_def == AppEntryDef::try_from(EntryTypesUnit::KeysetRoot).unwrap() {
                let entry_hashed = must_get_entry(entry_hash.clone())?;
                let ksr = KeysetRoot::try_from(entry_hashed)?;
                return Ok(SourceOfAuthority::KeysetRoot(ksr));
            }
            if *app_entry_def
                == AppEntryDef::try_from(EntryTypesUnit::DeviceInviteAcceptance).unwrap()
            {
                let entry_hashed = must_get_entry(entry_hash.clone())?;
                let dia = DeviceInviteAcceptance::try_from(entry_hashed)?;
                return Ok(SourceOfAuthority::DeviceInviteAcceptance(dia));
            }
        };
        Err(wasm_error!(WasmErrorInner::Guest(String::from(
            "Given ActionHash was neither a KeysetRoot nor a DeviceInviteAcceptance!"
        ))))
    })
}
