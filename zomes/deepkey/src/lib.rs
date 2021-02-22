use hdk::prelude::*;

mod derivation_path;
mod device_authorization;
mod dna_key_binding;
mod generator;
mod key;
mod key_anchor;
mod key_change_rule;
mod key_meta;
mod key_registration;
mod keyset_root;

entry_defs![
    derivation_path::DerivationPath::entry_def(),
    device_authorization::DeviceAuthorization::entry_def(),
    dna_key_binding::DnaKeyBinding::entry_def(),
    generator::Generator::entry_def(),
    key_change_rule::KeyChangeRule::entry_def(),
    key_registration::KeyRegistration::entry_def(),
    keyset_root::KeysetRoot::entry_def(),
]

