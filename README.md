
# Deepkey

Deepkey is a happ to provide a decentralized public key infrastructure (DPKI) for keys associated with Holochain conductors and applications. Similar to centralised services like Keybase, we want users to be able to manage their "keyset" by adding and removing devices and public/private keypairs.

The keys for happs installed on each device are also tracked under the keyset for the device.

Because humans are notoriously bad at managing cryptographic keys, we believe a project like Holochain must provide key management tools to help people deal with real-world messiness such as lost/stolen keys or devices. How many billions of dollars have been lost due to the lack of a key management infrastructure?

Deepkey provides the ability to:

- Register keys under the authority of a keyset.
- Replace keys with new ones.
- Revoke keys / declare them dead.
- Associate multiple devices under unified keyset management.
- Check the validity of a key.
- Store private instructions to rebuild app keys from a master seed to reestablish authority after data loss.
- Deepkey provides the ability to do social management of keys through m of n signatures (the initial default is a 1 of 1 signature using a revocation key).

Deepkey is a foundational app for all other Holochain app keys. Therefore, it is the first happ every conductor must install, and all other happs rely on it to query the status of keys.  It is designed to work hand in hand with holochain's secure keystore, [Lair](https://github.com/holochain/lair).

The most common call to Deepkey is `key_state((Key, Timestamp))` to query the validity of a key at a particular time.

## Interactions with Lair

Lair is designed to generate new keys from randomness, or generate new keys from a seed with derivation instructions.  In order for Deepkey to accomplish it's purpose of being able to regenerate your app keys from a device seed or a master seed for all of your devices, Deepkey must store the derivation patterns and instruct Lair to produce keys using them.  

### Workflows

- When you install Holochain, Deepkey needs to inject seeds (master, revocation, device) and provide a UI for password encrypting and [exporting your seeds](https://docs.rs/hc_seed_bundle/latest/hc_seed_bundle/) for off device storage.
- Every time a new Holochain app is installed, Deepkey must specify the derivation to generate the new agent keys in Lair, and store in a private entry the derivation and app DNA it was used with.
- Replacing compromised keys
- Deleting or revoking keys which have been abandoned
- Generating an invitation for a new device to join your keyspace
- Accepting an invitation for a new device
- Changing the rules for managing your keys
- Approving a key change or deletion, whether doing that locally with a revocation key or receiving that remotely as a social signing request

## Joining the DHT

The Deepkey `JoiningProof` involves two proofs. One is the membrane proof which is true for all happs, and the other is the keyset proof described in the next section.

### Membrane Proof

The purpose of a `membrane_proof` is to make it hard to flood the network with fake accounts.

_TODO: The membrane logic is not currently implemented._

Future membrane logic implementations planned:

- `ProofOfWork`: Agent must prove that they've performed some computational work to prevent low-effort spam bots.
- `ProofOfStake`: Agent must put up value that can be taken in case of bad behaviour.
- `ProofOfAuthority`: Agent must have a signature from a pre-defined authority to join.

There are a few external details to resolve before we require membrane proofs:

- Ability to have different versions of Deepkey apps to choose from, and configure their own joining proof.
- Ability for hosts to call external functions before joining the network, e.g. to generate a proof of work before completing installation of the app.
- More thought is needed about the types of membrane proofs we might like for the default behavior of Deepkey.

### Keyset Proof

The `keyset_proof` is the proof that the device has authority to participate in some keyset. The keyset defines the rules which determine the methods for revoking or replacing its keys.

The first entry the app makes in each user's source chain is a `KeysetRoot`, creating a new keyset space.

A source chain may later reference a valid `DeviceInvite`, in the form of a `DeviceInviteAcceptance`, to abandon the initial keyset and join another already existing keyset space.

(This will be at least the fifth entry in the chain, after the three genesis entries and the `init_complete`.)

If the keyset proof is a new `KeysetRoot` then it must be immediately followed by a valid `ChangeRule` to define how key management works within this keyset. On the other hand, if you join an existing keyset through a `DeviceInvite`, the `ChangeRule` of that keyset is what governs keys made on this chain.

## Keyset

A keyset is the set of keys governed by a ruleset, presumably under the control of one person.

When you install a new app in Holochain, by default a new keypair is generated to control the source chain of that app. The public key of that keypair serves as the address of your agent in that app's DHT. The private key signs all of your network communications and all actions on your chain. Deepkey registers, manages, and reports on the validity of each `AgentPubKey` installed on your conductor.

### Keyset Root

A `KeysetRoot` (KSR) is self-declared onto the network using a single-purpose throwaway keypair.

The structure of a `KeysetRoot` is:

- The `first_deepkey_agent` (FDA), the author of the `KeysetRoot`.
- The `root_pub_key`, the public part of a throwaway keypair which is only used to generate this KSR. (Using the `sign_ephemeral` HDK function.)
- A `Signature`: the authority of the FDA is established using the private part of the throwaway keypair to sign the FDA's pubkey.

Note that if a device is to issue a KSR it must do so as its very first action in Deepkey.

**A device can NEVER create another KSR without starting a new source chain.**

#### KeysetRoot API

**Create**: The validation that happens when you create a new `KeysetRoot`

- A `KeysetRoot` struct must deserialize cleanly from the record being validated.
- Must be created at index 4 (5th item) in the author's chain.
- The author must be the FDA.
- The signature of the FDA from the root/ephemeral pubkey must be valid.

**Read**: There is currently no read functions or lookups exposed as zome calls, but this may change in the future as people may want to use `KeysetRoot` as a unifying identity.

**Update**: Not allowed.

**Delete**: Not allowed.

**Zome Calls**:

- `create_keyset_root`
  - Input is a `(KeysetRoot, ChangeRule)` tuple.
  - Creates both the `KeysetRoot` and `ChangeRule` records sequentially.
  - Output is a `(ActionHash, ActionHash)` tuple of the created records.

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

### ChangeRule API

A `ChangeRule` defines the rules within a keyset for changing keys. It is used to validate replacement or revocation of any key. It can be configured to support social signing through m of n signatures of trusted agents, but by default it is configured as a 1 of 1 signature by a revocation key.

The structures involved in a `ChangeRule` are:

`AuthoritySpec` describes the authority/ies involved in replacing or revoking a key. Number of signatures required, and the public keys that are allowed to sign for the authorization.

```rust
pub struct AuthoritySpec {
    /// set to 1 for a single signer scenario
    pub sigs_required: u8,
    /// These signers probably do NOT exist on the DHT.
    /// E.g. a revocation key used to create the first change rule.
    pub authorized_signers: Vec<AgentPubKey>,
}
```

`AuthorizedSpecChange` exists to make a change to the authorization rules. It includes the new `AuthoritySpec`, and a set of authorizing signatures, valid according to the existing spec that this spec change replaces.

```rust
pub struct AuthorizedSpecChange {
    pub new_spec: AuthoritySpec,
    /// Signature of the content of the authority_spec field,
    /// signed by throwaway RootKey on Create,
    /// or according to previous authoritative AuthSpec upon Update.
    pub authorization_of_new_spec: Vec<Authorization>, // required sigs
}
```

`Authorization` is a tuple containing a u8 index into `authorized_signers`, and a valid signature from that key.

```rust
pub type Authorization = (u8, Signature);
```

A `ChangeRule` ties the new authorization spec to a KeysetRoot. It includes a proof of authority in the form of the `keyset_leaf`.

```rust
pub struct ChangeRule {
    pub keyset_root: ActionHash, // reference to a `KeysetRoot`
    pub keyset_leaf: ActionHash, // reference to either the `KeysetRoot` or a `DeviceInviteAcceptance` that proves the authority to change the rules for this Keyset
    pub spec_change: AuthorizedSpecChange, // defining the new multisig rules
}
```

A `ChangeRule` can only be _created_ immediately following a `KeysetRoot` entry on a source chain. `ChangeRule` records can be _updated_ on the chain of any device currently under the authority of the same `KeysetRoot`.

Note that the spec change signature validation does NOT require that all the signers exist as agents in Deepkey. This means hardware wallets, FIDO-compliant keys, smart cards, etc. could be used to provide signatures into your multisig.

The _create_ for a `ChangeRule` is expected to be signed by a "1 of 1" revocation key that is not the key of a Deepkey agent.

**Create**: The validation that happens when you create a new `ChangeRule`

- A `ChangeRule` must deserialize cleanly from the record being validated.
- A `KeysetRoot` must fetch and deserialize cleanly from the keyset root on the `ChangeRule`.
- The keyset leaf must be in the author's chain.
- There must NOT be any newer `DeviceInviteAcceptance` records in the validation package.
- The `KeysetRoot` FDA must be the author of the `ChangeRule`.
- The `ChangeRule` `prev_action` must be the `KeysetRoot` record.
- The `ChangeRule` authorization of the new spec must have exactly one authorization signature.
- The `ChangeRule` authorization signature must be valid as being from the `KeysetRoot` root (throwaway) pubkey.
- The `ChangeRule` `spec_change` specifies an 1 of 1 signing rule.
- In the `ChangeRule` spec, `sigs_required` = 1.

**Read**: There are no exposed zome function for looking up `ChangeRule`s.

**Update**: The validation that happens when you update a `ChangeRule`

- A `ChangeRule` must deserialize cleanly from the record being validated.
- A `KeysetRoot` must fetch and deserialize cleanly from the keyset root on the `ChangeRule`
- The previous `ChangeRule` must fetch and deserialize cleanly from the `original_action_address` of the update record.
- The previous `ChangeRule` record must be the original create record. (Every new `ChangeRule` updates the original `ChangeRule` record; that update action is referenced from the original `ChangeRule`, the fifth entry on the source chain.)
- The keyset leaf must be in the the author's chain.
- There must NOT be any newer `DeviceInviteAcceptance` records in the validation package.
- The `KeysetRoot` of the proposed `ChangeRule` must be the same as in the previous `ChangeRule`
- **The proposed `ChangeRule` authorization must authorize the new spec according to the rules of the previous `ChangeRule`**
- The `ChangeRule` `spec_change` specifies an m of n signing rule where n >= m.
- In the `ChangeRule` spec, `sigs_required` >= 1.

**Delete**: Not allowed.

#### Zome Calls

- `new_change_rule`:
  - The inputs are the `ActionHash` of the old change rule, and the new `ChangeRule`.
  - Updates the original `ChangeRule` entry (Create only happens when creating a `KeysetRoot` with the original throwaway key.)
  - Output is the `ActionHash` of the new change rule.

## Key Registration

Agents can register public keys on their source chain in Deepkey.

In the design space of all possible deepkey implementations, any type of public key could be registered and managed, e.g. for blockchains, TLS certificates, etc.

In the default Deepkey instance (the one that needs to be installed into a conductor so that other happs can register their keys in a shared space), only Holochain `AgentPubKey` public keys are supported for registration.

The `KeyRegistration` entry is the start of the process to manage a public key. There are other entry types that track and control how key registrations can work, i.e. which rules apply, which authority keys are registered under.

The `KeyAnchor` entry contains only the core 32 bytes of the registered key, stripped of the 3 byte multihash prefix and 4 byte DHT location suffix. Using this `KeyAnchor` entry, the status (valid, revoked, replaced, etc.) of a key can be looked up in a single `get_details` call, without needing to first lookup the corresponding `KeyRegistration`.

By default, Deepkey change rules support multisignature logic. This is through collecting multiple signatures and applying Holochain validation (not via a cryptographic threshold signature scheme). The `ChangeRule` defines the multisig rules that apply to all keys under the management of a `KeysetRoot`.

Key replacements or revocations must be fully authorized by a `ChangeRule` multisig.

### Generator API

A `Generator` is a special purpose key required for registering new keys. Holochain's default key store, "Lair" allows for layers of password encryption of keys. We introduced the concept of a `Generator` in order to make registering a new key require typing an additional password to unlock the `Generator` private key. This makes it so someone can't register a new key on your chain if they're sitting at your workstation with Holochain unlocked.

The structures comprising a `Generator` are:

```rust
pub struct Generator {
    change_rule: ActionHash, // `ChangeRule` action that authorizes this `Generator`
    change: Change,
}
pub struct Change {
    new_key: AgentPubKey, // A new special-purpose key being authorized as a `Generator`
    authorization: Vec<Authorization>, // authorizes the `new_key` according to the `ChangeRule` rules
}
```

**Create**: The validation that happens when you create a new `Generator`

- A `Generator` must deserialize cleanly from the record.
- The `change_rule` must fetch and deserialize cleanly from the referenced `ActionHash`.
- The `new_key` must be authorized by the authorization vec in the `Change` according to the `ChangeRule` rules.

**Read**: There is no read or lookup zome call exposed for `Generator`

**Update**: Not allowed

**Delete**: Not allowed

#### Zome Calls

- `new_generator`
  - input is a `Generator`
  - output is a `ActionHash`
  - creates a `Generator`

### KeyGeneration API

The structure of a `KeyGeneration` is:

```rust
pub struct KeyGeneration {
    new_key: AgentPubKey, // New key associated with current chain and KSR
    new_key_signing_of_author: Signature, // The new key must sign the Deepkey agent to join the Keyset
    // Ensure the generator has the same author as the KeyRegistration.
    generator: ActionHash, // This is the key authorized to generate new keys on this chain
    generator_signature: Signature, // The generator key signing the new key
}
```

#### Validation

- A `Generator` must fetch and deserialize cleanly for the `KeyGeneration` generator
- The `Generator` author must be the same as the `KeyGeneration` author
- The `Signature` of the `new_key` signing in the author of the `KeyGeneration` must be valid
- The `Signature` of the `new_key` from the `AgentPubKey` of the `Generator` must be valid

### KeyRegistration API

A `KeyRegistration` enum supports 4 ops/variants:

```rust
pub enum KeyRegistration {
    Create(KeyGeneration), // Creates a key under management of current KSR on this chain
    CreateOnly(KeyGeneration), // Keys for hosted web users may be of this type, cannot replace/revoke
    Update(KeyRevocation, KeyGeneration), // revokes a key and replaces it with a newly generated one
    Delete(KeyRevocation) // permanently revokes a key (Note: still uses an update action.)
}
```

`KeyGeneration` and `KeyRevocation` always use the same validation logic regardless of which `KeyRegistration` variant they are included in.

The `Delete` variant for a `KeyRegistration` uses the update action type for a Record. This is because delete actions don't register their change on the entry hash (just the action hash).

CRUD operations for a `KeyRegistration` must always be performed in the correct sequence with the corresponding CRUD operations for a `KeyAnchor`. Validation will enforce that the `KeyAnchor` is always preceded by its `KeyRegistration`.

**Note:** `CreateOnly` serves the temporary purpose of allowing Holo Hosts to register keys of web users without being able to manage those keys. This feature will most likely be replaced with adding a claim key for web users to claim their unmanaged keys if/when they become a self-hosted Holochain user. _TODO_

**Create**: The validation that happens when you create a new `KeyRegistration`

- A `KeyRegistration` must deserialize cleanly from the record
- The `KeyRegistration` must be a `Create` or `CreateOnly`
- The `KeyGeneration` must be valid

**Read**: No zome calls exposed for direct lookups. The status of a key is read by getting the `KeyAnchor` for a `KeyRegistration`.

**Update**: The validation that happens when you update a `KeyRegistration`

- A `KeyRegistration` must deserialize cleanly from the record
- The `KeyRegistration` must be an `Update` or `Delete`
- The prior key registration from the `KeyRevocation` must fetch and deserialize to a `KeyRegistration`
- The prior `KeyRegistration` must be a `Create` or `Update`
- The prior `Generator` must fetch and deserialize cleanly from the prior `KeyRegistration`
- The prior `ChangeRule` must fetch and deserialize cleanly from the prior `Generator`
- The proposed `KeyRevocation` must validate according to the prior `ChangeRule`
- If the `KeyRegistration` is an `Update` then the `KeyGeneration` must be valid

**Delete**:

- A `KeyRegistration` must fetch and deserialize cleanly from the `Delete` action's `deletes_address`
- The `KeyRegistration` must be a `KeyRegistration::Delete` variant
- The proposed `KeyRevocation` must validate according to the prior `ChangeRule`

#### Zome functions

- `new_key_registration`:
  - input is a `KeyRegistration`
  - IF the `KeyRegistration` is `Create` or `CreateOnly`
    - creates the `KeyRegistration` record
    - creates the `KeyAnchor` record
  - IF the `KeyRegistration` is `Update`
    - updates the prior key registration to the new key registration
    - looks up the revoked `KeyAnchor` and updates it to the new `KeyAnchor`
  - IF the `KeyRegistration` is `Delete`
    - updates the prior key registration to the new key registration
    - deletes that update
    - looks up the revoked `KeyAnchor` and deletes it

### KeyRevocation API

The structure of a `KeyRevocation` is:
```rust
pub struct KeyRevocation {
    prior_key_registration: ActionHash,
    // To be validated according to the change rule of the generator of the prior key.
    revocation_authorization: Vec<Authorization>,
}
```

#### Validation

- The `KeyRevocation` record must be an `Update` (Update the `KeyRegistration` entry with a `KeyRegistration::Update` or `KeyRegistration::Delete`)
- The `original_action_address` of the `Update` action must be the `prior_key_registration` of the `KeyRevocation`
- The prior change rule from the prior generator (see above) must `authorize` the prior `KeyRegistration` hash with the `KeyRevocation` authorization vec

### KeyAnchor API

The `KeyAnchor` entry contains only the core 32 bytes of the registered key, stripped of the 3 byte multihash prefix and 4 byte DHT location suffix. Using this `KeyAnchor` entry, the status (valid, revoked, replaced, etc.) of a key can be looked up in a single `get_details` call, without needing to first lookup the corresponding `KeyRegistration`.

This also means that any external consumer of Deepkey (other DNA's, Holochain apps, etc.) can query the key status with the core 32 bytes of the key. They do NOT need to know its registration details.

The `KeyAnchor` record must always be written onto the chain immediately following its `KeyRegistration`. This is to avoid the need to manage links between the two entries, the `KeyAnchor` `prev_action` always references the `KeyRegistration`.

**Create**: The validation that happens when you create a `KeyAnchor`

- A `KeyAnchor` must deserialize cleanly from the record
- A `KeyRegistration` must fetch and deserialize cleanly from the `KeyAnchor` prev action
- The `KeyRegistration` must be a `Create` or `CreateOnly` op
- The `KeyGeneration` new key's raw 32 bytes must be the `KeyAnchor` bytes

**Read**:`KeyAnchor` entries are designed to be looked up by hashing the core 32 bytes of a key. The `key_state` zome call returns relevant details about the status of key (valid, replaced, revoked, etc.)

**Update**: The validation that happens when you update a `KeyAnchor`

- A `KeyAnchor` must deserialize cleanly from the record
- A `KeyRegistration` must fetch and deserialize cleanly from the `KeyAnchor` prev action
- The `KeyRegistration` must be an `Update` op
- The `KeyGeneration` new key's raw 32 bytes must be the content of the `KeyAnchor` entry
- A `KeyAnchor` must fetch and deserialize cleanly from the `original_action_address` of the new `KeyAnchor` update action
- A `KeyRegistration` must fetch and deserialize cleanly from the prior key registration
- The revoked `KeyRegistration` must be a `Create` or `Update`
- The new key of the `KeyGeneration` of the revoked `KeyRegistration` must match the revoked `KeyAnchor` bytes

**Delete**:

- Must be able to fetch an `Record` from the `KeyAnchor` deletion record
- A `KeyAnchor` must fetch and deserialize from the `deletes_address` of the deletion record
- A `KeyRegistration` must fetch and deserialize from the `deletes_address` of the previous record
- The `KeyRegistration` must be an `Update` of type `KeyRegistration::Delete`
- The `KeyRevocation` from the `KeyRegistration` must be revoking the deleted `KeyAnchor`

#### Zome calls

- `key_state`:
  - input is `(KeyAnchor, Timestamp)` tuple
  - `Timestamp` doesn't do anything yet
  - output is `KeyState` which is `Valid/Invalidated/NotFound` as `SignedActionHashed`
    - If nothing found, `KeyState::NotFound` is returned
    - If any updates found, first update is returned in `KeyState::Invalidated`
    - If any deletes found, first delete is returned in `KeyState::Invalidated`
    - If any actions found, first action is returned in `KeyState::Valid`

## Private Metadata

In addition to shared/public keysets and registrations, each agent can keep private data for personal records about registered keys. This private data can be used to rebuild keypairs for apps from a master seed.

`KeyMeta` records record the derivation path and index used to generate a previously registered key.

`DnaBinding` records track how registered keys are being used by happs.

### Meta API

The structure of `KeyMeta` is:

- `new_key` referencing a `KeyRegistration` by its `ActionHash`
- `derivation_path` as 32 bytes encoding a derivation path for generating the registered key
- `derivation_index` as a u32 representing the index for generating the registered key from the `derivation_path`
- `key_type` as an enum of `AppUI`, `AppSig`, `AppEncryption`, `TLS` _TODO: confirm compatibility with Lair Key API_

**Create**:

- A `KeyMeta` must deserialize cleanly from the `Record`
- The `new_key` must fetch and deserialize to a `KeyRegistration` record
- The author of the `KeyMeta` and the `KeyRegistration` must be the same

**Read**: _TODO_

**Update**: Not allowed

**Delete**: Not allowed

#### Zome calls

- `new_key_meta`
  - input is `key_meta`
  - output is `ActionHash` of the created `KeyMeta`
  - creates a `KeyMeta`

### DnaBinding API

A `DnaBinding` is:

- A `key_meta` as `ActionHash` referencing a `KeyMeta`
- A `dna_hash` of the DNA the key is bound to
- An `app_name` as strings of `bundle_name` and `cell_nick` _TODO: make names compatible with new naming_

**Create**: A `DnaBinding` must deserialize cleanly from the `Record`

**Read**: _TODO_

**Update**: Not allowed

**Delete**: Not allowed

#### Zome calls

- `new_dna_binding`
  - input is `DnaBinding`
  - output is `ActionHash` of the created `DnaBinding`
  - creates a `DnaBinding`
- `install_an_app`
  - _TODO_

_TODO_: Discuss Rate Limiting
