use crate::{
    LinkTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // AnyLinkableHashTransformer,
    // verify_app_entry_struct,
    // Macros
    valid, // invalid,
};


pub fn validation(
    _base_address: AnyLinkableHash,
    _target_address: AnyLinkableHash,
    link_type: LinkTypes,
    _tag: LinkTag,
    _create: CreateLink,
) -> ExternResult<ValidateCallbackResult> {
    match link_type {
        LinkTypes::KeysetRootToKeyAnchors => {
            // verify_app_entry_struct::<WebAppPackageEntry>( &target_address )?;

            valid!()
        },
        LinkTypes::KSRToChangeRule |
        LinkTypes::KeysetRootToDeviceInviteAcceptances |
        LinkTypes::InviteeToDeviceInviteAcceptances |
        LinkTypes::DeviceInviteToDeviceInviteAcceptances |
        LinkTypes::DeviceToKeyAnchor |
        LinkTypes::DeviceName |
        LinkTypes::AppBindingToKeyMeta => {
            valid!()
        },
        // _ => invalid!(format!("Create link validation not implemented for link type: {:#?}", create.link_type )),
    }
}
