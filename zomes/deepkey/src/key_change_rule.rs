use hdk::prelude::*;

/// Represents an M:N multisignature spec.
/// The trivial case 1:1 represents a single agent to sign.
enum AuthoritySpec {
    sigs_required: u8,
    authorized_signers: Vec<AgentPubKey>,
}

struct AuthorizedSpecChange {
    new_spec: AuthoritySpec,
    /// Signature of the content of the authority_spec field,
    /// signed by throwaway RootKey on Create,
    /// or according to previous AuthSpec upon Update.
    authorization_of_new_spec: Vec<Signature>,
}

#[hdk_entry(id = "key_change_rule")]
struct KeyChangeRule {
    keyset_root: EntryHash,
    spec_change: AuthorizedSpecChange,
}

#[hdk_extern]
fn create_key_change_rule(new_key_change_rule: key_change_rule::KeyChangeRule) -> ExternResult<HeaderHash> {
    create_entry(new_key_change_rule)
}

#[hdk_extern]
fn update_key_change_rule(old_key_change_rule: HeaderHash, new_key_change_rule: key_change_rule::KeyChangeRule) -> ExternResult<HeaderHash> {
    update_entry(old_key_change_rule, new_key_change_rule)
}