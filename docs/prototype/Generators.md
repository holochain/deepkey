[back to CONTRIBUTING.md](../../CONTRIBUTING.md)


## Generators

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
