[back to CONTRIBUTING.md](../../CONTRIBUTING.md)


## Joining the DHT

The Deepkey `JoiningProof` involves two proofs. One is the membrane proof which is true for all
happs, and the other is the keyset proof described in the next section.

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
