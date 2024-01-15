mod validation;

pub mod authority_spec;
pub mod authorized_spec_change;
pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod device_name;
pub mod dna_binding;
pub mod error;
pub mod joining_proof;
pub mod key_anchor;
pub mod key_meta;
pub mod key_registration;
pub mod key_revocation;
pub mod key_generation;
pub mod keyset_root;
pub mod source_of_authority;

pub use key_anchor::*;
pub use joining_proof::*;
pub use key_registration::*;
pub use key_revocation::*;
pub use key_generation::*;
pub use device_invite_acceptance::*;
pub use device_invite::*;
pub use change_rule::*;
pub use authorized_spec_change::*;
pub use authority_spec::*;
pub use keyset_root::*;
pub use error::*;
pub use source_of_authority::*;
pub use device_name::*;
pub use key_meta::*;
pub use dna_binding::*;

use hdi::prelude::*;
use hdi_extensions::{
    // guest_error,
    scoped_type_connector,
    ScopedTypeConnector,
};


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_defs]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    KeysetRoot(KeysetRoot),

    ChangeRule(ChangeRule),

    DeviceInvite(DeviceInvite),
    DeviceInviteAcceptance(DeviceInviteAcceptance),

    KeyRegistration(KeyRegistration),
    KeyAnchor(KeyAnchor),

    #[entry_def(visibility = "private")]
    KeyMeta(KeyMeta),
    #[entry_def(visibility = "private")]
    DnaBinding(DnaBinding),
}

scoped_type_connector!(
    EntryTypesUnit::KeysetRoot,
    EntryTypes::KeysetRoot( KeysetRoot )
);
scoped_type_connector!(
    EntryTypesUnit::ChangeRule,
    EntryTypes::ChangeRule( ChangeRule )
);


#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    ChangeRuleUpdates,
    KeysetRootToDeviceInviteAcceptances,
    KeysetRootToKeyAnchors,
    InviteeToDeviceInviteAcceptances,
    DeviceInviteToDeviceInviteAcceptances, // unused for now
    DeviceName,
}
