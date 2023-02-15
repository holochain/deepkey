use crate::*;
use hdi::prelude::*;

#[hdk_entry_helper]
#[derive(Clone)]
pub enum SourceOfAuthority {
    KeysetRoot(KeysetRoot),
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_extern]
pub fn get_source_of_authority(
    (agent, from_action_hash): (AgentPubKey, ActionHash),
) -> ExternResult<Option<SourceOfAuthority>> {
    let filter = ChainFilter::new(from_action_hash);
    let activities = must_get_agent_activity(agent, filter)?;

    let keyset_root_app_def = AppEntryDef::try_from(UnitEntryTypes::KeysetRoot).unwrap();
    let device_invite_acceptance_app_def =
        AppEntryDef::try_from(UnitEntryTypes::DeviceInviteAcceptance).unwrap();
    for activity in activities.into_iter() {
        if let Some(EntryType::App(app_entry_def)) = activity.action.action().entry_type() {
            if *app_entry_def == keyset_root_app_def {
                let entry_hashed =
                    must_get_entry(activity.action.action().entry_hash().unwrap().clone())?;
                let ksr = KeysetRoot::try_from(entry_hashed)?;
                return Ok(Some(SourceOfAuthority::KeysetRoot(ksr)));
            }
            if *app_entry_def == device_invite_acceptance_app_def {
                let entry_hashed =
                    must_get_entry(activity.action.action().entry_hash().unwrap().clone())?;
                let dia = DeviceInviteAcceptance::try_from(entry_hashed)?;
                return Ok(Some(SourceOfAuthority::DeviceInviteAcceptance(dia)));
            }
        };
    }
    Ok(None)
}
