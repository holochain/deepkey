[back to CONTRIBUTING.md](../../CONTRIBUTING.md)


## Multi-device Management

### Keyset Proof

The `keyset_proof` is the proof that the device has authority to participate in some keyset. The keyset defines the rules which determine the methods for revoking or replacing its keys.

The first entry the app makes in each user's source chain is a `KeysetRoot`, creating a new keyset space.

A source chain may later reference a valid `DeviceInvite`, in the form of a `DeviceInviteAcceptance`, to abandon the initial keyset and join another already existing keyset space.

(This will be at least the fifth entry in the chain, after the three genesis entries and the `init_complete`.)

If the keyset proof is a new `KeysetRoot` then it must be immediately followed by a valid `ChangeRule` to define how key management works within this keyset. On the other hand, if you join an existing keyset through a `DeviceInvite`, the `ChangeRule` of that keyset is what governs keys made on this chain.


### Keyset Tree/Leaves

Under each `KeysetRoot` is an arbitrary tree of `DeviceInvite` and `DeviceInviteAcceptance` pairs as entries. A `DeviceInviteAcceptance` brings a device under the management of the `KeysetRoot` that it references. This has the effect of saying, "the same entity that created the keyset also controls this device."

**Each device can only be managed by a single keyset at one time.**

Accepting an invite moves ownership to a new entity, and removes the device along with its keys from the previous keyset. This is why both the invitor and invitee need to commit corresponding entries to their chains. The invitation contain cryptographic signatures of the process of transferring ownership.

The structure of a `DeviceInvite` (written to the invitor's chain) is:

- KSR: An `ActionHash` referring to the invitor's KSR.
- Parent: An `ActionHash` referring to the invitor's direct parent in the keyset tree, which is either its KSR or its current `DeviceInviteAcceptance`. This is used to establish the chain of authority from the original KSR.
- Invitee: The `AgentPubKey` being invited.

The structure of a `DeviceInviteAcceptance` (written to the invitee's chain) is:

- The `ActionHash` of the KSR.
- The `ActionHash` of the `DeviceInvite`.

#### Device Invite API

**Create**: The validation that happens when you create a new `DeviceInvite`:

- A `DeviceInvite` must deserialize cleanly from the validating record.
- The KSR must be fetched and deserialized into a `KeysetRoot`.
- An invitee must have a different `AgentPubkey` than the invitor.
- If the author of the invitation is the FDA in the invitation's KSR
  - Do a hash-bounded query from the invite hash back to the KSR in the invitor's source chain.
  - Check that that range contains no invite acceptances (have abandoned the Keyset they are inviting a new device into).
- Else (author of invitation and FDA of KSR are not the same):
  - Search from invite backwards (must_get_agent_activity of the invitor), find the first `DeviceInviteAcceptance` in their chain.
  - The invite in that `DeviceInviteAcceptance` must fetch and deserialize to a `DeviceInvite`.
  - That deserialized `DeviceInvite` must have the same KSR authority as the new `DeviceInvite` currently being validated.
  - Also in that `DeviceInvite`, the invitee must be the author of the new `DeviceInvite`.

We do not check whether the invitee exists on the DHT yet because they likely don't, that's why we're inviting them. If the `DeviceInviteAcceptance` is valid, and the `DeviceInvite` is valid, we trust that the parent's `DeviceInviteAcceptance` was properly validated, which ensures chain of authority to the KSR.

**Read**: No direct read or lookup functions exposed in zome calls. The keyset tree structure is used internally for validation of key registration/revocation logic.

**Update**: Not allowed.

**Delete**: Not allowed.

**Zome Calls**:

- `invite_agent`
  - Input is the `AgentPubKey` to invite.
    - This agent does not exist on the DHT yet if they are planning to use the invite as their joining proof.
  - Output is the exact `DeviceInviteAcceptance` the invitee must commit to their chain.
  - Invites are always under the current keyset.

#### Device Invite Acceptance API

**Create**:

- A `DeviceInviteAcceptance` must deserialize cleanly from the validating record
- A `DeviceInvite` must be fetched and deserialize from the `invite` action hash on the `DeviceInviteAcceptance`
- The author of the `DeviceInviteAcceptance` must be the referenced `AgentPubKey` on the `DeviceInvite`
- The `KeysetRoot` must be the same on both the `DeviceInvite` and the `DeviceInviteAcceptance`

**Read**: No exposed zome calls for read or lookup. For validation, the most recent `DeviceInviteAcceptance` is used to determine the current keyset.

**Update**: Not allowed.

**Delete**: Not allowed.

##### Zome Calls

- `accept_invite`
  - input is a `DeviceInviteAcceptance`
  - output is the `ActionHash` of the entry created
  - creates the entry as-is from input
