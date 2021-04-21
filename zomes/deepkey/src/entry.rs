use hdk::prelude::*;

use crate::change_rule::entry::ChangeRule;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::keyset_root::entry::KeysetRoot;
use crate::dna_binding::entry::DnaBinding;
use crate::generator::entry::Generator;
use crate::key_registration::entry::KeyRegistration;

entry_defs![
    ChangeRule::entry_def(),
    DeviceInvite::entry_def(),
    DeviceInviteAcceptance::entry_def(),
    KeysetRoot::entry_def(),
    DnaBinding::entry_def(),
    Generator::entry_def(),
    KeyRegistration::entry_def()
];