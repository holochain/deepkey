#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(with = "serde_bytes")]
pub struct PubKey {
    /// Bitcoin, Ethereum
    secp256k1([u8; 64]),
    /// Bitcoin, Schnorr
    Schnorr([u8; 33]),
    /// Polkadot https://wiki.polkadot.network/docs/en/learn-cryptography
    Sr25519([u8; 32]),
    /// Holochain, WebAuth, Keybase https://en.wikipedia.org/wiki/EdDSA#Ed25519
    Ed25519([u8; 32]),
}

