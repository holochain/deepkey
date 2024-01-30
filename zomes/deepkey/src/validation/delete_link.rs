use crate::{
    LinkTypes,
};

use hdi::prelude::*;
use hdi_extensions::{
    // Macros
    valid, invalid,
};


pub fn validation(
    original_action_hash: ActionHash,
    _base_address: AnyLinkableHash,
    _delete: DeleteLink,
) -> ExternResult<ValidateCallbackResult> {
    let record = must_get_valid_record( original_action_hash )?;
    let create_link = match record.action() {
        Action::CreateLink(action) => action,
        _ => invalid!(format!("Original action hash does not belong to create link action")),
    };
    let link_type = match LinkTypes::from_type( create_link.zome_index, create_link.link_type )? {
        Some(lt) => lt,
        None => invalid!(format!("No match for LinkTypes")),
    };

    match link_type {
        LinkTypes::KeysetRootToKeyAnchors => {
            invalid!(format!("KeysetRootToKeyAnchors links cannot be deleted"))
        },
        LinkTypes::KSRToChangeRule => {
            invalid!(format!("KSRToChangeRule links cannot be deleted"))
        },
        LinkTypes::KeysetRootToDeviceInviteAcceptances => {
            invalid!(format!("KeysetRootToDeviceInvites links cannot be deleted"))
        },
        LinkTypes::InviteeToDeviceInviteAcceptances => {
            invalid!(format!("InviteeToDeviceInvites links cannot be deleted"))
        },
        LinkTypes::DeviceInviteToDeviceInviteAcceptances => {
            invalid!(format!("DeviceInviteToDeviceInviteAcceptances links cannot be deleted"))
        },
        LinkTypes::DeviceToKeyAnchor => {
            invalid!(format!("DeviceToKeyAnchor links cannot be deleted"))
        },
        LinkTypes::DeviceName => {
            valid!()
        },
        LinkTypes::AppBindingToKeyMeta => {
            valid!()
        },
        // _ => {
        //     // if create_link.author != delete.author {
        //     //     invalid!(format!(
        //     //         "Not authorized to delete link created by author {}",
        //     //         create_link.author
        //     //     ))
        //     // }

        //     valid!()
        // },
    }
}
