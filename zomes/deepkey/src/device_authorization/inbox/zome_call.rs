use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::entry::LinkTypes;
use deepkey_integrity::device_authorization::device_invite_acceptance::error::Error;

use crate::device_authorization::device_invite::zome_call::*;

#[hdk_extern]
fn invite_agents_and_notify(invitees: Vec<AgentPubKey>) -> ExternResult<()> {
    let acceptances = invite_agents(InviteAgentsInput{invitees: invitees.clone()})?;
    for (invitee, acceptance) in invitees.into_iter().zip(acceptances.into_iter()) {
        match get(
            acceptance.invite,
            GetOptions::latest(),
        )? {
            Some(record) => match record.entry() {
                RecordEntry::Present(entry) => {
                    create_link(
                        invitee.to_owned(),
                        hash_entry(entry.to_owned())?,
                        LinkTypes::AgentInviteNotify,
                        (),
                    )?;
                },
                _ => return Err(Error::InviteNotFound.into()),
            },
            _ => return Err(Error::InviteNotFound.into()),
        }
    };
    Ok(())
}
