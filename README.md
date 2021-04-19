# Deepkey

Deepkey is a happ to track public keys associated with devices and other happs.

Similar to centralised services like Keybase, we want users to be able to add and remove devices to their "account".

We don't really have accounts, because deepkey is a happ, but we do have the concept of a "keyset".

The keys for happs installed on each device are also tracked under the keyset for the device.

Deepkey supports real-world messiness such as lost/stolen devices, multisignature key revocation and immutable/mutable keys registrations.

Deepkey is the foundational app for all other happs.

It is the first happ every conductor must install, and all other happs rely on it to query the status of keys.

The 32 byte representation of any pubkey can be used to query its status in a single DHT query.

## Keyset

A keyset is the _set_ of _keys_ controlled by a _single entity_ (ostensibly a human).

The keys in a keyset are the `AgentPubKey` of the source chain authors in deepkey.

Each source chain in deepkey is modelled as a real-world device, e.g. a laptop, mobile, etc.

It could also be a more abstract entity such as a fleet of IoT devices.

When a new device joins the network it must either start a new keyset or prove it has been invited to an existing keyset.


