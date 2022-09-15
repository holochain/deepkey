use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::device_authorization::device_invite::entry::DeviceInvite;
use deepkey_integrity::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use deepkey_integrity::device_authorization::inbox::DEVICE_INVITE_LINK_TAG_BYTES;
use deepkey_integrity::entry::{ EntryTypes, LinkTypes };

use crate::device_authorization::device_invite::local_keyset_parent;

#[derive(Debug, Deserialize)]
pub struct InviteAgentsInput {
    pub invitees: Vec<AgentPubKey>,
}

#[hdk_extern]
pub fn invite_agents(input: InviteAgentsInput) -> ExternResult<Vec<DeviceInviteAcceptance>> {
    debug!("invite_agents: {:?}", input);
    let mut acceptances: Vec<DeviceInviteAcceptance> = Vec::new();
    for invitee in input.invitees.into_iter() {
        let (keyset_root, parent) = local_keyset_parent()?;
        let invite = DeviceInvite::new(
            keyset_root.clone(),
            parent,
            invitee.clone(),
        );
        let invite_header = create_entry(EntryTypes::DeviceInvite(invite.clone()))?;
        create_link(
            invitee,
            hash_entry(invite)?,
            LinkTypes::AgentInvite,
            LinkTag(DEVICE_INVITE_LINK_TAG_BYTES.iter().chain(invite_header.get_raw_39().iter()).cloned().collect::<Vec<u8>>()),
        )?;
        acceptances.push(DeviceInviteAcceptance::new(
            keyset_root.clone(),
            invite_header,
        ));
    }
    debug!("invite_agents acceptances: {:?}", acceptances);
    Ok(acceptances)
}
