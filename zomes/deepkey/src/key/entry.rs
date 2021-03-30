use hdk::prelude::*;
#[cfg(test)]
use ::fixt::prelude::*;

const ED25519_KEY_LEN: usize = 32;
const SECP256K1_KEY_LEN: usize = 64;
const SCHNORR_KEY_LEN: usize = 33;
const SR25519_KEY_LEN: usize = 32;

#[derive(Clone, Copy, Debug)]
pub struct Ed25519([u8; ED25519_KEY_LEN]);

impl AsRef<[u8]> for Ed25519 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
fixturator!(
    Ed25519;
    curve Empty Ed25519(fixt!(ThirtyTwoBytes, Empty));
    curve Unpredictable Ed25519(fixt!(ThirtyTwoBytes, Unpredictable));
    curve Predictable Ed25519(fixt!(ThirtyTwoBytes, Predictable));
);

#[derive(Clone, Copy, Debug)]
pub struct Secp256k1([u8; SECP256K1_KEY_LEN]);

impl AsRef<[u8]> for Secp256k1 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
fixturator!(
    Secp256k1;
    curve Empty Secp256k1(fixt!(SixtyFourBytes, Empty));
    curve Unpredictable Secp256k1(fixt!(SixtyFourBytes, Unpredictable));
    curve Predictable Secp256k1(fixt!(SixtyFourBytes, Predictable));
);

#[derive(Clone, Copy, Debug)]
pub struct Schnorr([u8; SCHNORR_KEY_LEN]);

impl AsRef<[u8]> for Schnorr {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
pub type ThirtyThreeBytes = [u8; 33];

#[cfg(test)]
fixturator!(
    ThirtyThreeBytes;
    curve Empty [0; 33];
    curve Unpredictable {
        let bytes: Vec<u8> = (0..33).map(|_| rand::random::<u8>()).collect();
        let mut ret = [0; 33];
        ret.copy_from_slice(&bytes);
        ret
    };
    curve Predictable {
        let mut u8_fixturator = U8Fixturator::new_indexed(Predictable, get_fixt_index!());
        let mut bytes = vec![];
        for _ in 0..33 {
            bytes.push(u8_fixturator.next().unwrap());
        }
        let mut ret = [0; 33];
        ret.copy_from_slice(&bytes);
        ret
    };
);

#[cfg(test)]
fixturator!(
    Schnorr;
    curve Empty Schnorr(fixt!(ThirtyThreeBytes, Empty));
    curve Unpredictable Schnorr(fixt!(ThirtyThreeBytes, Unpredictable));
    curve Predictable Schnorr(fixt!(ThirtyThreeBytes, Predictable));
);

#[derive(Clone, Copy, Debug)]
pub struct Sr25519([u8; SR25519_KEY_LEN]);

impl AsRef<[u8]> for Sr25519 {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[cfg(test)]
fixturator!(
    Sr25519;
    curve Empty Sr25519(fixt!(ThirtyTwoBytes, Empty));
    curve Unpredictable Sr25519(fixt!(ThirtyTwoBytes, Unpredictable));
    curve Predictable Sr25519(fixt!(ThirtyTwoBytes, Predictable));
);

fixed_array_serialization!(Ed25519, ED25519_KEY_LEN);
fixed_array_serialization!(Secp256k1, SECP256K1_KEY_LEN);
fixed_array_serialization!(Schnorr, SCHNORR_KEY_LEN);
fixed_array_serialization!(Sr25519, SR25519_KEY_LEN);

#[hdk_entry(id = "key")]
#[derive(Clone)]
pub enum Key {
    /// Holochain, WebAuth, Keybase https://en.wikipedia.org/wiki/EdDSA#Ed25519
    Ed25519(Ed25519),
    /// Bitcoin, Ethereum
    Secp256k1(Secp256k1),
    /// Bitcoin, Schnorr
    Schnorr(Schnorr),
    /// Polkadot https://wiki.polkadot.network/docs/en/learn-cryptography
    Sr25519(Sr25519),
}

#[cfg(test)]
fixturator!(
    Key;
    variants [ Ed25519(Ed25519) Secp256k1(Secp256k1) Schnorr(Schnorr) Sr25519(Sr25519) ];
);

impl AsRef<[u8]> for Key {
    fn as_ref(&self) -> &[u8] {
        match self {
            Key::Ed25519(ed25519) => ed25519.as_ref(),
            Key::Secp256k1(secp256k1) => secp256k1.as_ref(),
            Key::Schnorr(schnorr) => schnorr.as_ref(),
            Key::Sr25519(sr25519) => sr25519.as_ref(),
        }
    }
}