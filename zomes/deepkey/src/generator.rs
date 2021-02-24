use hdk::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct AuthorizedKeyChange {
    new_key: AgentPubKey,
    authorization_of_new_key: Vec<Signature>,
}

#[hdk_entry(id = "generator")]
pub struct Generator {
    key_change_rule: EntryHash,
    key_change: AuthorizedKeyChange,
}

#[hdk_extern]
fn create_generator(new_generator: Generator) -> ExternResult<HeaderHash> {
    create_entry(new_generator)
}

#[hdk_extern]
fn update_generator((old_generator, new_generator): (HeaderHash, Generator)) -> ExternResult<HeaderHash> {
    update_entry(old_generator, new_generator)
}

#[hdk_extern]
fn delete_generator(old_generator: HeaderHash) -> ExternResult<HeaderHash> {
    delete_entry(old_generator)
}