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

