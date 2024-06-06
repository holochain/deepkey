[![](https://img.shields.io/crates/v/hc_deepkey_types?style=flat-square&label=types)](https://crates.io/crates/hc_deepkey_types)
[![](https://img.shields.io/crates/v/hc_deepkey_sdk?style=flat-square&label=sdk)](https://crates.io/crates/hc_deepkey_sdk)

# Deepkey
A DNA to provide a decentralized public key infrastructure (DPKI) for keys associated with Holochain
conductors and applications. Similar to centralised services like Keybase, we want users to be able
to manage their "keyset" by adding and removing public/private keypairs.

[![](https://img.shields.io/github/issues-raw/holochain/deepkey?style=flat-square)](https://github.com/holochain/deepkey/issues)
[![](https://img.shields.io/github/issues-closed-raw/holochain/deepkey?style=flat-square)](https://github.com/holochain/deepkey/issues?q=is%3Aissue+is%3Aclosed)
[![](https://img.shields.io/github/issues-pr-raw/holochain/deepkey?style=flat-square)](https://github.com/holochain/deepkey/pulls)

See [Pre-2024 Architectual Documentation](docs/2023/README.md) for original design principles.


## Overview
The keys for happs installed on a device are also tracked under the keyset for the device.

Because humans are notoriously bad at managing cryptographic keys, we believe a project like
Holochain must provide key management tools to help people deal with real-world messiness such as
lost/stolen keys or devices. How many billions of dollars have been lost due to the lack of a key
management infrastructure?

Deepkey is a foundational app for all other Holochain app keys. Therefore, it is the first happ
every conductor must install, and all other happs rely on it to query the status of keys.  It is
designed to work hand in hand with holochain's secure keystore,
[Lair](https://github.com/holochain/lair).

The most common call to Deepkey is `key_state((Key, Timestamp))` to query the validity of a key at a
particular time.


### Features
Deepkey provides the ability to:

- Register keys under the authority of a keyset.
- Define the revocation rules for a keyset.
- Replace keys with new ones.
- Revoke keys / declare them dead.
- Check the validity of a key.
- Store private instructions to rebuild app keys from a master seed to reestablish authority after
  data loss.

Future features

- Associate multiple devices under unified keyset management.
- Do social management of keys through m of n signatures.


## How it works?

Workflows

- Initializing Deepkey
- Registering a new key
- Replacing a key
- Revoking a key
- Checking key validity
- Updating the change rules


#### Interactions with Lair

> Although Deepkey is designed to work with Lair, there is no direct dependence on Lair as all
> interactions with Lair are facilitated by the DPKI service in Holochain.

Lair is designed to generate new keys from randomness, or generate new keys from a seed with
derivation instructions.  In order to regenerate your app keys, Deepkey must store the derivation
info that will instruct Lair to reproduce the same keys.


#### How are keys determinitic?

Key uniquenss is determined by a combination of 2 incrementing numbers; an **app index** and a **key
index**.  When an app is installed, a new key registration is made using the next unused app index
and a key index of `0`.  Replacing keys will increment the key index while the app index stays the
same.


### Initializing Deepkey

When you install Holochain, Deepkey will create a new Keyset Root (KSR) as the first action.  Then
it will set the initial change rules for that KSR and register itself as the first key (ie. app/key
index `0/0`).

> *There can only be one KSR on a Deepkey source chain.*

#### What is a KSR?

A `KeysetRoot` (KSR) is self-declared onto the network using a single-purpose throwaway keypair.

The structure of a `KeysetRoot` is:

- `first_deepkey_agent` - (FDA) the author of the `KeysetRoot`.
- `root_pub_key` - the public part of a throwaway keypair which is only used to generate this KSR
- `signed_fda` - the authority of the FDA is established by signing thd FDA's pubkey using the
  private part of the throwaway keypair


### Registering a new key

Registering a new key is a 2 step process; generating the new key outside of Deepkey and then
registering it in Deepkey.

1. In order to generate the key, Deepkey provides a way to get the "next derivation details" which
   can be used to generate a deterministic key.
2. Then that key can be committed in Deepkey along with all the other registration input such as
   details about the app and derivation input.

App and derivation details are committed as private entries.


### Replacing a key

Key replacement is simply a combination of key revocation and key generation committed together.  It
requires most of the same steps from [Registering a new key](#registering-a-new-key) and [Revoking a
key](#revoking-a-key).

1. In order to generate the next key, Deepkey provides a way to get the "next derivation details"
   given a specific key which will return the existing `app_index` for that key and the next unsused
   `key_index`.
2. Gather the required signatures for revoking the previous key registration (see [Revoking a
   key](#revoking-a-key)).
3. Then the new key can be committed as a replacement to the previous key.

Derivation details are committed as a private entry.


### Revoking a key

The act of revoking a key (without a key update) ends the evolution of that key.  This means that
all app chains using that key will not no longer be valid as of the revocation commit timestamp.

A revocation is done by signing the `KeyRegistration`'s `ActionHash` with the number of keys
required by the current change rules.


### Checking key validity

A key state can be checked using the key bytes and a timestamp.  There are 3 states a key can be in

- Valid
  - the timestamp is after a key create action timestamp and before any delete action timestamps
- Invalid
  - the timestamp is before any key create action timestamps
  - or, the timestamp is after a key create action timestamp and after a delete action timestamp
- Not found
  - there are no create or delete actions



### Updating the change rules

A `ChangeRule` defines the rules within a keyset.  It can be configured to support signing with M of
N keys (ie. an `AuthoritySpec`) which is used to validate changes to keys and the change rules
themselves.

Initially, the change rule is has an `AuthoritySpec` set to 1 of 1 with the only authority being the
FDA.

A change rule can be updated by constructing a new `AuthoritySpec`, signing it with the required
keys (based on the existing change rule), and then commiting the new `ChangeRule` as an update to
the original.

> *There can only be one `ChangeRule` create action on a Deepkey source chain.  Any following
> `ChangeRule` commits must be udpates to the original.*
