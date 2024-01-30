mod validation;

pub mod utils;
pub mod authority_spec;
pub mod authorized_spec_change;
pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod device_name;
pub mod app_binding;
pub mod error;
pub mod joining_proof;
pub mod key_anchor;
pub mod key_meta;
pub mod key_registration;
pub mod keyset_root;
pub mod source_of_authority;

pub use authorized_spec_change::*;
pub use authority_spec::*;
pub use change_rule::*;
pub use device_invite_acceptance::*;
pub use device_invite::*;
pub use device_name::*;
pub use app_binding::*;
pub use error::*;
pub use joining_proof::*;
pub use keyset_root::*;
pub use key_anchor::*;
pub use key_meta::*;
pub use key_registration::*;
pub use source_of_authority::*;

use serde_bytes;
use hdi::prelude::*;
use hdi_extensions::{
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
    AppBinding(AppBinding),
}

scoped_type_connector!(
    EntryTypesUnit::KeysetRoot,
    EntryTypes::KeysetRoot( KeysetRoot )
);
scoped_type_connector!(
    EntryTypesUnit::ChangeRule,
    EntryTypes::ChangeRule( ChangeRule )
);
scoped_type_connector!(
    EntryTypesUnit::DeviceInvite,
    EntryTypes::DeviceInvite( DeviceInvite )
);
scoped_type_connector!(
    EntryTypesUnit::DeviceInviteAcceptance,
    EntryTypes::DeviceInviteAcceptance( DeviceInviteAcceptance )
);
scoped_type_connector!(
    EntryTypesUnit::KeyRegistration,
    EntryTypes::KeyRegistration( KeyRegistration )
);
scoped_type_connector!(
    EntryTypesUnit::KeyAnchor,
    EntryTypes::KeyAnchor( KeyAnchor )
);
scoped_type_connector!(
    EntryTypesUnit::KeyMeta,
    EntryTypes::KeyMeta( KeyMeta )
);
scoped_type_connector!(
    EntryTypesUnit::AppBinding,
    EntryTypes::AppBinding( AppBinding )
);


#[derive(Serialize, Deserialize)]
#[hdk_link_types]
pub enum LinkTypes {
    KSRToChangeRule,
    KeysetRootToDeviceInviteAcceptances,
    KeysetRootToKeyAnchors,
    InviteeToDeviceInviteAcceptances,
    DeviceInviteToDeviceInviteAcceptances, // unused for now
    DeviceToKeyAnchor,
    DeviceName,
    AppBindingToKeyMeta,
}



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct MembraneProof {
    pub joining_proof: serde_bytes::ByteBuf,
}
