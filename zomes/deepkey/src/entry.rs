use hdk::prelude::*;

entry_defs![
    keyset_root::entry::KeysetRoot::entry_def(),
    device_authorization::entry::DeviceAuthorization::entry_def(),
    change_rule::entry::ChangeRule::entry_def(),
    dna_key_binding::DnaKeyBinding::entry_def(),
    generator::Generator::entry_def(),
    key_registration::KeyRegistration::entry_def()
];