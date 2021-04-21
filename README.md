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

## Key registration

Agents can register public keys on their source chain in deepkey.

In the general sense of all possible deepkey implementations, the public keys can be any type of public key, e.g. for blockchains or other systems.

In the "main" deepkey, or at least the one that needs to be installed into a conductor so that other happs can register their keys in a shared space, only holochain `AgentPubKey` public keys are supported for registration.

The `KeyRegistration` entry is the main entry point into managing public keys.

There are other entry types that track and control how key registrations can work.

The `KeyAnchor` entry is the literal bytes of the registered key so that the status of a key can be looked up in a single `get` call, without needing to first lookup the corresponding `KeyRegistration`.

The `ChangeRule` defines multisig rules.

A `Generator` that has been signed off by the multisig of a `ChangeRule` to authorize the author of the `Generator` to register new keys.

Key revocations must always be fully authorized by a `ChangeRule` multisig.

### KeyRegistration API

A `KeyRegistration` enum supports 4 ops/variants:

- `Create` includes a `KeyGeneration` and can be updated or deleted
- `CreateOnly` includes a `KeyGeneration` but can never be updated or deleted
- `Update` includes a `KeyRevocation` and a `KeyGeneration` and must be an Update header
- `Delete` includes a `KeyRevocation` and must be an `Update` header

`KeyGeneration` and `KeyRevocation` validation logic always work the same regardless of which op they are included in.

The op is used to align the `Entry` intent with the `Element` header.

Note that `Delete` op for a `KeyRegistration` is aligned with the `Update` header for an `Element`. This is because `Delete` headers cannot be associated with an `Entry`.

Note also that the CRUD operations for a `KeyRegistration` must always be performed in the correct sequence with the corresponding CRUD operations for a `KeyAnchor`. This is enforced by happ validation.

#### Create

- A `KeyRegistration` must deserialize cleanly from the element
- The `KeyRegistration` must be a `Create` or `CreateOnly`
- The `KeyGeneration` must be valid

#### Read

- not intended for direct lookups
- the status of a key is read by getting the `KeyAnchor` for a `KeyRegistration`

#### Update

- A `KeyRegistration` must deserialize cleanly from the element
- The `KeyRegistration` must be an `Update` or `Delete`
- The prior key registration from the `KeyRevocation` must fetch and deserialize to a `KeyRegistration`
- The prior `KeyRegistration` must be a `Create` or `Update`
- The prior `Generator` must fetch and deserialize cleanly from the prior `KeyRegistration`
- The prior `ChangeRule` must fetch and deserialize cleanly from the prior `Generator`
- The proposed `KeyRevocation` must validate according to the prior `ChangeRule`
- IF the `KeyRegistration` is an `Update` then the `KeyGeneration` must be valid

#### Delete

- A `KeyRegistration` must fetch and deserialize cleanly from the `Delete` header's `deletes_address`
- The `KeyRegistration` must be a `KeyRegistration::Delete` variant

#### Zome functions

- `new_key_registration`:
  - input is a `KeyRegistration`
  - IF the `KeyRegistration` is `Create` or `CreateOnly`
    - creates the `KeyRegistration` element
    - creates the `KeyAnchor` element
  - IF the `KeyRegistration` is `Update`
    - updates the prior key registration to the new key registration
    - looks up the revoked `KeyAnchor` and updates it to the new `KeyAnchor`
  - IF the `KeyRegistration` is `Delete`
    - updates the prior key registration to the new key registration
    - deletes that update
    - looks up the revoked `KeyAnchor` and deletes it

### KeyGeneration API

`KeyGeneration` is:

- the `new_key` being associated with the deepkey author
- the `new_key_signing_of_author` as a `Signature` that the new key accepts being associated with the deepkey author
  - note that there is NOT a nonce under this `Signature` so the signing is permanent
- the `generator` as a `HeaderHash` to the `Generator` entry defining the acceptance rules
- the `generator_signature` as a `Signature` from the generator's `AgentPubKey`

#### validation

- A `Generator` must fetch and deserialize cleanly for the `KeyGeneration` generator
- The `Generator` author must be the same as the `KeyGeneration` author
- The `Signature` of the `new_key` signing in the author of the `KeyGeneration` must be valid
- The `Signature` of the `new_key` from the `AgentPubKey` of the `Generator` must be valid

### KeyRevocation API

`KeyRevocation` is:

- The `prior_key_registration` being revoked
- The `revocation_authorization` as a `Vec<Authorization>`

#### validation

- The `KeyRevocation` element must be an `Update`
- The `original_header_address` of the `Update` header must be the `prior_key_registration` of the `KeyRevocation`
- The prior change rule from the prior generator (see above) must `authorize` the prior `KeyRegistration` with the `KeyRevocation` authorization vec

### KeyAnchor API

A `KeyAnchor` is the literal `32` bytes of a pubkey.

This means that any external consumer of deepkey can query the key status with the `32` bytes of the key only. They do NOT need to know the registration details, or holochainisms such as key prefixes for an `AgentPubKey`.

The `KeyAnchor` element must always pair with its `KeyRegistration` element. This is to avoid the need to manage links between the two entries, the `KeyAnchor` element header always directly references the `KeyRegistration`.

#### Create

- A `KeyAnchor` must deserialize cleanly from the element
- A `KeyRegistration` must fetch and deserialize cleanly from the `KeyAnchor` prev header
- The `KeyRegistration` must be a `Create` or `CreateOnly` op
- The `KeyGeneration` new key's raw 32 bytes must be the `KeyAnchor` bytes

#### Read

- `KeyAnchor` entries are designed to be looked up by their literal bytes
- The `key_state` zome call fetches relevant details about the `KeyAnchor`

#### Update

- A `KeyAnchor` must deserialize cleanly from the element
- A `KeyRegistration` must fetch and deserialize cleanly from the `KeyAnchor` prev header
- The `KeyRegistration` must be an `Update` op
- The `KeyGeneration` new key's raw 32 bytes must be the `KeyAnchor` bytes
- A `KeyAnchor` must fetch and deserialize cleanly from the `original_header_address` of the new `KeyAnchor` update header
- A `KeyRegistration` must fetch and deserialize cleanly from the prior key registration
- The revoked `KeyRegistration` must be a `Create` or `Update`
- The new key of the `KeyGeneration` of the revoked `KeyRegistration` must match the revoked `KeyAnchor` bytes

#### Delete

- Must be able to fetch an `Element` from the `KeyAnchor` deletion element
- A `KeyAnchor` must fetch and deserialize from the `deletes_address` of the deletion element
- A `KeyRegistration` must fetch and deserialize from the `deletes_address` of the previous element
- The `KeyRegistration` must be an `Update` of type `KeyRegistration::Delete`
- The `KeyRevocation` from the `KeyRegistration` must be revoking the deleted `KeyAnchor`

#### Zome calls

- `key_state`:
  - input is `(KeyAnchor, Timestamp)` tuple
  - `Timestamp` doesn't do anything yet
  - output is `KeyState` which is `Valid/Invalidated/NotFound` as `SignedHeaderHashed`
  - IF any updates found, first update is returned in `KeyState::Invalidated`
  - IF any deletes found, first delete is returned in `KeyState::Invalidated`
  - IF any headers found, first header is returned in `KeyState::Valid`
  - IF nothing found `KeyState::NotFound` is returned

### ChangeRule API

A `ChangeRule` is:

- A `keyset_root` reference to the `HeaderHash` of a `KeysetRoot`
- A `keyset_leaf` reference to the `HeaderHash` of either the `KeysetRoot` or a `DeviceInviteAcceptance`
- A `spec_change` defining the new multisig rules

A `ChangeRule` can only be created immediately after a `KeysetRoot`.

`ChangeRule` elements can be updated by any device currently under the same `KeysetRoot`.

#### Create

- A `ChangeRule` must deserialize cleanly from the element
- A `KeysetRoot` must fetch and deserialize cleanly from the keyset root on the `ChangeRule`
- The keyset leaf must be in the validation package (the author's chain)
- There must NOT be any newer `DeviceInviteAcceptance` elements in the validation package
- The `KeysetRoot` FDA must be the author of the `ChangeRule`
- The `ChangeRule` prev header must be the `KeysetRoot` element
- The `ChangeRule` authorization of the new spec must have exactly one authorization signature
- The `ChangeRule` authorization signature must be valid as from the `KeysetRoot` FDA
- The `ChangeRule` spec must have more or equal signers to required signatures
- The `ChangeRule` must require at least one signature

#### Read

- `ChangeRule` elements are not directly looked up

#### Update

- A `ChangeRule` must deserialize cleanly from the element
- A `KeysetRoot` must fetch and deserialize cleanly from the keyset root on the `ChangeRule`
- A `ChangeRule` must fetch and deserialize cleanly from the `original_header_address` of the update element
- Every signer from the authorized signers must fetch and deserialize cleanly to an `AgentPubKey`
- __The previous `ChangeRule` element must be a `Create` element (flat CRUD tree)__
- The keyset leaf must be in the validation package (the author's chain)
- There must NOT be any newer `DeviceInviteAcceptance` elements in the validation package
- The `KeysetRoot` of the proposed `ChangeRule` must be the same as the previous `ChangeRule`
- __The proposed `ChangeRule` authorization must authorize the new spec according to the rules of the previous `ChangeRule`__
- The proposed `ChangeRule` must have more or equal signers to required signatures
- The proposed `ChangeRule` must require at least one signature

#### Delete

n/a

#### Zome calls

- `new_change_rule`:
  - __input is `HeaderHash` of old change rule and new `ChangeRule`__
  - updates the entry
    - create must be done when creating a `KeysetRoot`
  - output is `HeaderHash` of new change rule

### Generator API

A `Generator` is:

- The `HeaderHash` of a `ChangeRule` that authorizes this `Generator`
- A new `AgentPubKey` being authorized as a `Generator`
- A `Vec<Authorization>` that authorizes the `AgentPubKey` according to the `ChangeRule` rules

#### Create

- A `Generator` must deserialize cleanly from the element
- A `ChangeRule` must fetch and deserialize cleanly from the referenced `HeaderHash`
- The `AgentPubKey` must be authorized by the authorization vec on the `Generator` according to the `ChangeRule` rules

#### Read

- there is no read or lookup functionality for `Generator`
- it is used internally to validate `KeyRegistration` key generations

#### Update

n/a

#### Delete

n/a

#### Zome calls

- `new_generator`
  - input is a `Generator`
  - output is a `HeaderHash`
  - creates a `Generator`

