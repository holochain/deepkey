use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::entry::LinkTypes;

use crate::device_authorization::device_invite::zome_call::invite_agents;

#[hdk_extern]
fn invite_agents_and_notify(invitees: Vec<AgentPubKey>) -> ExternResult<()> {
    let acceptances = invite_agents(invitees.clone())?;
    for (invitee, acceptance) in invitees.iter().zip(acceptances.iter()) {
        create_link(
            invitee,
            hash_entry(
                get(
                    acceptance.invite,
                    GetOptions::latest(),
                )?
            ),
            LinkTypes::AgentInviteNotify,
        )
    }
}
