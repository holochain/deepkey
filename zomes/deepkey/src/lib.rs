mod validation;

pub mod utils;
pub mod error;
pub mod joining_proof;
pub mod source_of_authority;

// Re-exports
pub use deepkey_types;

pub use deepkey_types::*;
pub use error::*;
pub use joining_proof::*;
pub use source_of_authority::*;

use hdi::prelude::*;
use hdi_extensions::{
    scoped_type_connector,
    ScopedTypeConnector,
};


#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
#[hdk_entry_types]
#[unit_enum(EntryTypesUnit)]
pub enum EntryTypes {
    KeysetRoot(KeysetRoot),

    ChangeRule(ChangeRule),

    KeyRegistration(KeyRegistration),
    KeyAnchor(KeyAnchor),

    #[entry_type(visibility = "private")]
    KeyMeta(KeyMeta),
    #[entry_type(visibility = "private")]
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
    KeysetRootToKeyAnchors,
    DeviceToKeyAnchor,
    DeviceName,
    AppBindingToKeyMeta,
}
