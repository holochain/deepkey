# Deepkey

Deepkey is a happ to track public keys associated with devices and other happs.

Similar to centralised services like Keybase, we want users to be able to add and remove devices to their "account".

We don't really have accounts, because deepkey is a happ, but we do have the concept of a "keyset".

The keys for happs installed on each device are also tracked under the keyset for the device.

Deepkey supports real-world messiness such as lost/stolen devices, multisignature key revocation and immutable/mutable keys registrations.

Deepkey is the foundational app for all other happs.

It is the first happ every conductor must install, and all other happs rely on it to query the status of keys.

The 32 byte representation of any pubkey can be used to query its status in a single DHT query.

## Joining the DHT

The deepkey `JoiningProof` is two proofs.

### Key set proof

The `keyset_proof` is the proof that the device can exist in some key set (see below).

The validation and CRUD logic for the key set is implemented.

It must be either a valid `KeysetRoot` or a reference to a valid `DeviceInvite` as a `DeviceInviteAcceptance`.

If the keyset proof is a new `KeysetRoot` then it must be immediately followed by a valid `ChangeRule` to define how public key management works within the key set (see below).

### Membrane proof

The `membrane_proof` is the proof that makes it harder to flood the network with garbage.

The membrane logic is NOT implemented.

Future ideas include:

- `ProofOfWork`: Agent must hit some difficulty to prevent low effort spam bots
- `ProofOfStake`: Agent must put up value that can be slashed in case of bad behaviour
- `ProofOfAuthority`: Agent must have a signature from known authorities to join

There are a few external details to resolve before the membrane proofs would make sense:

- Ability to have different deepkeys for happs to choose and configure proof
- Ability for hosts to call extern functions before joining the network to e.g. generate a proof of work consistent with the verification
- More thought about the types of membrane proofs we might like for deepkey

## Keyset

A key set is the _set_ of _keys_ controlled by a _single entity_ (ostensibly a human).

The keys in a keyset are the `AgentPubKey` of the source chain authors in deepkey.

Each source chain in deepkey is modelled as a real-world device, e.g. a laptop, mobile, etc.

It could also be a more abstract entity such as a fleet of IoT devices.

When a new device joins the network it must either start a new keyset or prove it has been invited to an existing keyset.

It does this by committing either a `KeysetRoot` or `DeviceInviteAcceptance` as the fourth entry on the chain.

### Key set root

The `KeysetRoot` (KSR) is:

- The `first_deepkey_agent` or "FDA" which is the `AgentPubKey` of the agent that is authoring the `KeysetRoot`
- The `root_pub_key` is the `AgentPubKey` of a keypair that is immediately discarded with `sign_ephemeral` from the HDK
- The `Signature` of the `first_deepkey_agent` by the ephemeral private key of the `root_pub_key`

This self-signs a new entity into the network.

Note that if a device is to issue a KSR it must do so as its very first action in deepkey. A device can accept an invite to join a different key set (see below).

__A device can NEVER create a second KSR without starting a new source chain.__

In general there is nothing stopping sybils/spammers from self-signing themselves into Deepkey. That needs to be mitigated by a combination of:

- the joining proof and membrane
- keeping the app as lightweight as possible in general
- rate limits to mitigate spam: https://hackmd.io/5HfCCyLgScCPqVI-s8uSjA

#### KeysetRoot API

##### Create

- A `KeysetRoot` struct must deserialize cleanly from the element being validated
- must be created at index 3 (4th position) in the author's chain
- the author must be the FDA
- the signature of the FDA from the root/ephermal pubkey must be valid

##### Read

- no direct read functions or lookups exposed in zome calls
- a new invite issued by any device will include its KSR header hash

##### Update

n/a

##### Delete

n/a

##### Zome calls

- `create_keyset_root`
  - input is a `(KeysetRoot, ChangeRule)` tuple
  - creates both the `KeysetRoot` and `ChangeRule` sequentially
  - output is a `(HeaderHash, HeaderHash)` tuple of the created elements

### Key set tree/leaves

Under each `KeysetRoot` is an arbitrary tree of `DeviceInvite` and `DeviceInviteAcceptance` pairs as entries.

A valid `DeviceInviteAcceptance` brings a device under the specific `KeysetRoot` that it references.

This has the effect of saying "The same entity that created the keyset also owns/controls this device".

__Each device can only be under a single key set at one time.__

Accepting an invite has the effect of moving ownership to a new entity and removing ownership from the previous entity. This is why both the invitor and invitee need to commit entries to their chain, the headers contain cryptographic signatures of the process of transferring ownership.

A `DeviceInvite` is:

- A `HeaderHash` reference to its KSR
- A `HeaderHash` reference to its direct parent in the tree
  - Either the KSR or a `DeviceInviteAcceptance`
- The `AgentPubKey` being invited

A `DeviceInviteAcceptance` is:

- The `HeaderHash` of the KSR
- The `HeaderHash` of the `DeviceInvite`

#### Device Invite API

##### Create

- A `DeviceInvite` must deserialize cleanly from the validating element
- The KSR must be fetched and deserialized into a `KeysetRoot`
- An author cannot invite themselves
- The parent element must be fetched
- There must be no `DeviceInviteAcceptance` headers in the full chain validation package between the parent header and the current element
- If the parent and KSR are the same `HeaderHash` then:
  - The author must be the FDA of the KSR
- Else:
  - The parent must fetch and deserialize to a `DeviceInviteAcceptance`
  - The deserialized parent's invite must fetch and deserialize to a `DeviceInvite`
  - The deserialized `DeviceInvite` must have the same KSR as the new `DeviceInvite`
  - The deserialized `DeviceInvite` invitee must be the author of the new `DeviceInvite`

__We do NOT check the invitee exists on the DHT yet because they likely don't, that's why we're inviting them.__

##### Read

- no direct read or lookup functions exposed in zome calls
- key sets are mostly used internally for key registration/revocation logic

##### Update

n/a

##### Delete

n/a

##### Zome calls

- `invite_agent`
  - input is the `AgentPubKey` to invite
    - this agent does not exist on the DHT yet if they are planning to use the invite as their joining proof
  - output is the exact `DeviceInviteAcceptance` the invitee must commit to their chain
  - invites are always under the current key set

#### Device Invite Acceptance API

##### Create

- A `DeviceInviteAcceptance` must deserialize cleanly from the validating element
- A `DeviceInvite` must be fetched and deserialize from the `invite` header hash on the `DeviceInviteAcceptance`
- The author of the `DeviceInviteAcceptance` must be the referenced `AgentPubKey` on the `DeviceInvite`
- The `KeysetRoot` must be the same on both the `DeviceInvite` and the `DeviceInviteAcceptance`

##### Read

- no direct read or lookup functions exposed in zome calls
- internally the most recent `DeviceInviteAcceptance` on the author's chain is always used to determine the current key set

##### Update

n/a

##### Delete

n/a

##### Zome calls

- `accept_invite`
  - input is a `DeviceInviteAcceptance`
  - output is the `HeaderHash` of the entry created
  - creates the entry as-is from input