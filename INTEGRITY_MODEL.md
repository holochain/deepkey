[back to README.md](README.md)


# Integrity Model

The purpose of this document is to describe the intentions that guide the development of Deepkey's
integrity zome.  It assumes an understanding of the purposes and use-cases of Deepkey so that the
validation criteria can be concise and comprehensible.  See [README.md](README.md) for more context.

### Entity Relationship Diagram (high-level)

![](docs/images/ERD.png)


## `KeysetRoot` Validation

### Create

Validation criteria

- The record must deserialize into the correct struct
- Must be created at index 3 (4th item) in the author's chain.
  - This ensures there could only be one create per-chain
- The author must be the FDA.
- The signature (ie. `signed_fda`) must be authored by the root/ephemeral key (ie. `root_pub_key`)
  signing the FDA (ie. `first_deepkey_agent`).


### Read
Allowed

### Update
Not allowed.

### Delete
Not allowed.



## `ChangeRule` Validation

Summary of the change rule entry

- **KSR** - identifies which Keyest root the change rules affect
- **Spec change** (ie. `AuthorizedSpecChange`)
  - **New spec** (ie. `AuthoritySpec`) - describes the authority/ies required for change that
    include the number of signatures required, and the public keys that are allowed to sign for the
    authorization.
    - **Number of signature required** - the value of M for "M of N" signing
    - **Authorized signers** - the list of public keys that make the value of N for "M of N" signing
  - **Authorization of new spec** (ie. `Authorization`) - list of authorizations from current
    authorized signers signers
    - an index indicating which key was used to make the signature
    - the signature of the new spec's serialized bytes signed by a current authorized signer

### Create

Validation criteria

- The record must deserialize into the correct struct
- Must be created at index 4 (5th item) in the author's chain.
  - This ensures there could only be one create per-chain
  - The previous record will be the KSR create
- The author must be the FDA.
- The new spec is a 1 of 1 where the FDA is the only authorized signer

### Read
Allowed

### Update

Validation criteria

- The record must deserialize into the correct struct
- The `original_action_hash` must be the create record on the same source chain
- The `keyset_root` cannot be changed
- New spec requirements
  - The signatures required cannot be 0
  - The signatures required cannot be more than the number of authorized signers
- Authorization requirements - *the 'new spec' from the previous change rule is applied for this
  change*
  - The 'number of authorizations' for the new spec exceeds the 'signatures required' by the
    previous change rule spec
  - Each authorization has a valid signature matching an authorized signer from the previous change
    rule
    - Signed content will be the new `authority_spec`

### Delete
Not allowed.



## `KeyRegistration` Validation

> **Note:** Key generation and key revocation always use the same validation logic regardless of
> which variant they are in.

### Key generation

Validation criteria

- Signature of action author by new key is valid

### Key revocation
*The latest change rule spec is applied*

Validation criteria

- The 'number of authorizations' exceeds the 'signatures required' by the change rule spec
- Each authorization has a valid signature matching an authorized signer from the change rule spec
  - Signed content will be the prior key registration action address

### Create

Validation criteria

- The record must deserialize into the correct struct
- Must be a `Create` or `CreateOnly` variant
- [Key generation requirements](#key-generation)

### Read
Allowed

### Update

Validation criteria

- The record must deserialize into the correct struct
- Must be a `Update` or `Delete` variant
- The `original_action_hash` must be the `prior_key_registration` of the revocation
- There must not be any other `KeyRegistration` on the same source chain referencing the same
  `prior_key_registration`
- [Key revocation requirements](#key-revocation)
- If variant is an `Update`
  - [Key generation requirements](#key-generation)

### Delete
Not allowed.



## `KeyAnchor` Validation

CRUD operations must always be performed in the correct sequence.  Validation will enforce that the
`KeyAnchor` is always preceded by its `KeyRegistration`.

### Create

Validation criteria

- The record must deserialize into the correct struct
- The previous action must be the `KeyRegistration` that generated this key
- The key registration must be a `Create` or `CreateOnly` variant

### Read
Allowed

> Key anchor addresses are designed to be deterministic using the core 32 bytes of a key

### Update

Validation criteria

- The record must deserialize into the correct struct
- The previous action must be the `KeyRegistration` that generated this key
- The key registration must be the `Update` variant
- The key registration's revoked address must match the `original_action_hash`

### Delete

Validation criteria

- The previous action must be the `KeyRegistration` that is deleting this key
- The key registration must be the `Delete` variant
- The key registration's revoked address must match the `original_action_hash`



## `AppBinding` Validation

### Create

Validation criteria

- The app index should be an increment of 1 based on the latest `AppBinding` on the same source
  chain

### Read
Requires access to the source chain because it is a private entry.

### Update
Not allowed.

### Delete
Not allowed.



## `KeyMeta` Validation
Records the derivation path and index used to generate a previously registered key.

### Create

Validation criteria

- The key index should be an increment of 1 based on the previous `KeyMeta` for the same app binding
- The referenced key anchor should be the one created by the referenced key registration

### Read
Requires access to the source chain because it is a private entry.

### Update
Not allowed.

### Delete
Not allowed.
