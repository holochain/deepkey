// Use case: A is root and B has a device invite bundle
// - A is fine
// - B commits the invite acceptance in position 4 and can join the network because it references the DeviceInvite
//
// Use case: A is root, B is root but wants to move to A
// - A is fine
// - B needs to receive a DeviceInvite from A somehow and then commit a DeviceInviteAcceptance that will replace its root
//
// Note we don't need to represent rejecting an invite, we can simply ignore it because there's no harm.
// For example a device cannot be stolen, then an attacker accepts a nascent invite then go on a revoking spree
// because the revocation private key is sitting behind a password-locked lair doing the private key management.
// Also because accepting the invite would put _more_ control back in the hands of the stealee not the stealor.
//
// "Somehow" getting an invite from A:
// - 'inbox' pattern
// - optimistically A sends a remote call to N recipients as vec![B, C, D, ..]
// - any remote calls that fail fallback to a link on the recipient's key
// - when recipient finds the link it deletes the link _after_ it processes the inbox message (e.g. accepts device invitation)
// - validation that only recipient can delete inbox link
// - in the future we can have an ephemeral store for the inbox that cleans up for real and not just leaving links
// - API functions:
//   - `send` - sends a message to recipients
//   - `check` - recipient checks their own inbox for new messages
//   - `mark_read` - recipient deletes their own link, so it won't show up in subsequent `check` calls
//
// A sends to B's inbox
// B interacts with some GUI e.g. in holochain itself to check the inbox
// B either accepts or ignores the invite
// - if accept,
//    - commit `DeviceInvitance` entry
//    - B adds a link from the KSA to itself, and puts the header hash of the device invite acceptance in the link tag for validation
//    - then `mark_read`
// - if ignore, do nothing and `mark_read`

// #[hdk_entry(id = "device_authorization")]
// pub struct DeviceAuthorization {
//     /// Each DeviceAuthorization comes under a singe KeysetRoot as the 'authority'
//     keyset_root_authority: HeaderHash,
//     /// DeviceAuthorization entries form a tree up to a KeysetRoot on the `root_acceptance` side.
//     /// If the parent references the `AgentPubKey` in the child's `root_acceptance` then we know by induction
//     /// that the root agent in this entry comes under the `keyset_root_authority`.
//     /// At the top of the tree the parent will be the KeysetRoot in which case the DA
//     /// on the KSR chain must have a `root_acceptance` agent that is the FDA and the DA/KSR authors match.
//     parent: HeaderHash,
//     root_acceptance: Acceptance,
//     device_acceptance: Acceptance,
// }