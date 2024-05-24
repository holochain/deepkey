
DL (May 20, 2024)

# Key Authority Declaration

## Context

Due to the [Change rule circumvention issue](https://github.com/holochain/deepkey/issues/23), we
must commit additional information on the app chains to designate which change rules are the
trusted authority for managing a key.


## Options

### 1. `key_authority: AgentPubKey`

Identifies the deepkey agent who is the authority for key updates.

This is used to determine which change rules must be followed for updating the keys.

#### Validation process when claiming an app agent

- Check that the `KeyAnchor` in deepkey has a create made by the `key_authority` agent.

#### Validation process when updating an app agent

- Lookup the previous agent record to get the `key_authority`
- Check that the `KeyAnchor` in deepkey has a create made by the `key_authority` agent.
- Check that the valid create is an update to the previous key.

#### Questions

#####  A. Does deepkey's validation guarantee that the existence of the create means it is a valid udpate?

Deepkey's validation will guarantee that the anchor create was preceeded by a valid registration
(ie. the registration had all the valid signatures).

#####  B. What happens when the malicious deepkey tries to update the key?

The validation will fail because the malicious deepkey agent does not batch the `key_authority`


### 2. `key_authority: ActionHash`

Points to the KSR authority.

#### Validation process when claiming an app agent

- Check that the `KeyAnchor` in deepkey has a create made by an agent that is a member of the
  `key_authority` (KSR)
  - Eg. for each create, check that their KSR, or the latest invite acceptance on their chain, matches the KSR
    authority.

#### Validation process when updating an app agent

- Lookup the previous agent record to get the `key_authority`
- Check that the `KeyAnchor` in deepkey has a create made by an agent that is a member of the
  `key_authority` (KSR)
  - Eg. for each create, check that their KSR, or the latest invite acceptance on their chain, matches the KSR
    authority.
- Check that the valid create is an update to the previous key.

#### Questions

#####  A. Does deepkey's validation guarantee that the existence of the create means it is a valid udpate?

Deepkey's validation will guarantee that the anchor create was preceeded by a valid registration
(ie. the registration had all the valid signatures).

However, question C. needs to be solved to know that the registration followed the latest change
rules.

#####  B. What happens when the malicious deepkey tries to update the key?

The validation will fail because they will not have a KSR or invite acceptance that matches the
`key_authority`.


#####  C. How do we know that the update followed the latest change rules?

Since joining a KSR is essentially the same as copying the change rules, could we solve this by
treating invite acceptance as an "auto-copy" declaration?
