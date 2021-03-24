use hdk::prelude::*;

use crate::change_rule::entry::ChangeRule;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_accepted::entry::DeviceInviteAccepted;
use crate::keyset_root::entry::KeysetRoot;
use crate::dna_binding::entry::DnaKeyBinding;
use crate::generator::entry::Generator;
use crate::key_registration::entry::KeyRegistration;

entry_defs![
    ChangeRule::entry_def(),
    DeviceInvite::entry_def(),
    DeviceInviteAccepted::entry_def(),
    KeysetRoot::entry_def(),
    DnaKeyBinding::entry_def(),
    Generator::entry_def(),
    KeyRegistration::entry_def()
];