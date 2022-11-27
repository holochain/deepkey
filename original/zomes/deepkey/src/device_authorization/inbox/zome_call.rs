use hdk::prelude::*;

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
            )
        )
    }
}
