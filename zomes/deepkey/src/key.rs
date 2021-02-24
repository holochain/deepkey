use hdk::prelude::*;

const ED25519_KEY_LEN: usize = 32;
const SECP256K1_KEY_LEN: usize = 64;
const SCHNORR_KEY_LEN: usize = 33;
const SR25519_KEY_LEN: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct Ed25519([u8; ED25519_KEY_LEN]);

#[derive(Clone, Copy, Debug)]
pub struct Secp256k1([u8; SECP256K1_KEY_LEN]);

#[derive(Clone, Copy, Debug)]
pub struct Schnorr([u8; SCHNORR_KEY_LEN]);

#[derive(Clone, Copy, Debug)]
pub struct Sr25519([u8; SR25519_KEY_LEN]);

fixed_array_serialization!(Ed25519, ED25519_KEY_LEN);
fixed_array_serialization!(Secp256k1, SECP256K1_KEY_LEN);
fixed_array_serialization!(Schnorr, SCHNORR_KEY_LEN);
fixed_array_serialization!(Sr25519, SR25519_KEY_LEN);

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum PubKey {
    /// Holochain, WebAuth, Keybase https://en.wikipedia.org/wiki/EdDSA#Ed25519
    Ed25519(Ed25519),
    /// Bitcoin, Ethereum
    Secp256k1(Secp256k1),
    /// Bitcoin, Schnorr
    Schnorr(Schnorr),
    /// Polkadot https://wiki.polkadot.network/docs/en/learn-cryptography
    Sr25519(Sr25519),
}
