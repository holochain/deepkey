

# Main methods with description

## Getting information
- `query_keyset_authority_action_hash`: Gets the keyset authority (KSR actionhash) (either device invite acceptance or keyset root) for this conductor
- `query_keyset_members`: Get the AgentPubKey of every member of a Keyset. (Everyone who's written a device invite acceptance, or who has the KSR on their chain.)
- `query_keyset_keys`: Get all the KeyAnchors for every key registered on this Keyset

## Inviting
`invite_agent`: The agent to invite into the keyset. Returns an unsigned DeviceInviteAcceptance.


# Flat List of all public DeepKey hdk_extern methods

## authority_spec.rs
- create_authority_spec
- get_authority_spec
- get_authority_specs_for_signer

## authorized_spec_change.rs
- create_authorized_spec_change
- get_authorized_spec_change

## change_rule.rs
- create_change_rule
- get_change_rule
- update_change_rule

## device_invite_acceptance.rs
- create_device_invite_acceptance
- get_device_invite_acceptance
- get_device_invite_acceptances_for_device_invite
- accept_invite

## device_invite.rs
- create_device_invite
- get_device_invite
- get_device_invites_for_keyset_root
- get_device_invites_for_invitee
- invite_agent

## joining_proof.rs
- create_joining_proof
- get_joining_proof

## key_anchor.rs
- key_state
- get_key_registration_from_agent_pubkey_key_anchor
- get_agent_pubkey_key_anchor

## key_generation.rs
- get_key_generation

## key_registration.rs
- new_key_registration
- update_key

## key_revocation.rs
- instantiate_key_revocation
- authorize_key_revocation
- revoke_key
- get_key_revocation
- update_key_revocation

## keyset_root.rs
- create_keyset_root
- get_keyset_root

## source_of_authority.rs
- query_keyset_authority_action_hash
- query_keyset_root_action_hash


