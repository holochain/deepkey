use hdk::prelude::*;

use crate::change_rule::entry::ChangeRule;
use crate::device_authorization::device_invite::entry::DeviceInvite;
use crate::device_authorization::device_invite_acceptance::entry::DeviceInviteAcceptance;
use crate::keyset_root::entry::KeysetRoot;
use crate::dna_binding::entry::DnaBinding;
use crate::generator::entry::Generator;
use crate::key_registration::entry::KeyRegistration;

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(name = "joining_proof")]
    JoiningProof(JoiningProof), // TODO: Confirm; was previously absent in entry_types! table
    #[entry_def(name = "change_rule")] //, required_validation_type = "full")
    ChangeRule(ChangeRule),
    #[entry_def(name = "device_invite")] //, required_validation_type = "full")
    DeviceInvite(DeviceInvite),
    #[entry_def(name = "device_invite_acceptance")]
    DeviceInviteAcceptance(DeviceInviteAcceptance),
    #[entry_def(name = "keyset_root")]
    KeysetRoot(KeysetRoot),
    #[entry_def(name = "key_meta", visibility="private")]
    KeyMeta(KeyMeta),
    #[entry_def(name = "dna_binding", visibility="private")]
    DnaBinding(DnaBinding),
    #[entry_def(name = "generator")]
    Generator(Generator),
    #[entry_def(name = "key_registration")]
    KeyRegistration(KeyRegistration),
}

    
/*
entry_types!([
    ChangeRule::entry_def(),
    DeviceInvite::entry_def(),
    DeviceInviteAcceptance::entry_def(),
    KeysetRoot::entry_def(),
    DnaBinding::entry_def(),
    Generator::entry_def(),
    KeyRegistration::entry_def()
]);
*/
