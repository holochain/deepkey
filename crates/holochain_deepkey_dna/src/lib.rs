//! Just exports the bytes of the canonical Deepkey DNA bundle.

/// Get the hard-coded Deepkey DNA provided by this crate.
/// This can be decoded with `holochain_types::DnaBundle::decode()`
pub const DEEPKEY_DNA_BUNDLE_BYTES: &[u8] = include_bytes!("deepkey.dna");

pub mod types {
    pub use deepkey_sdk::*;
}

pub use deepkey_sdk::hdk;
