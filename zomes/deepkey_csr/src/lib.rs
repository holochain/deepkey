pub mod change_rule;
pub mod device;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod device_name;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod source_of_authority;
pub mod app_binding;
pub mod key_meta;
pub mod utils;

pub use hdk_extensions;
pub use hdk_extensions::hdi_extensions;

use deepkey::*;
use hdi_extensions::{
    guest_error,
};
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
};
use keyset_root::create_keyset_root;


#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
    // TODO: This cannot actually be done here because we must pass in the revocation keys.  Unless
    // we separate the KSR creation and authority spec creation.  Which means the ephemeral key
    // would not be able to sign the authority spec.
    create_keyset_root(())?;

    // grant unrestricted access to receive_device_invitation so other agents can send us device invitations
    let mut fns = BTreeSet::new();
    fns.insert((zome_info()?.name, "receive_device_invitation".into()));
    let functions = GrantedFunctions::Listed(fns);
    create_cap_grant(CapGrantEntry {
        tag: "".into(),
        // empty access converts to unrestricted
        access: ().into(),
        functions,
    })?;

    Ok(InitCallbackResult::Pass)
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Signal {
    EntryCreated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
    },
    EntryUpdated {
        action: SignedActionHashed,
        app_entry: EntryTypes,
        original_app_entry: EntryTypes,
    },
    EntryDeleted {
        action: SignedActionHashed,
        original_app_entry: EntryTypes,
    },
    LinkCreated {
        action: SignedActionHashed,
        link_type: LinkTypes,
    },
    LinkDeleted {
        action: SignedActionHashed,
        link_type: LinkTypes,
    },
    InvitationReceived {
        device_invite_acceptance: Vec<u8>,
    },
}


#[hdk_extern(infallible)]
pub fn post_commit(committed_actions: Vec<SignedActionHashed>) {
    for action in committed_actions {
        if let Err(err) = signal_action(action) {
            error!("Error signaling new action: {:?}", err);
        }
    }
}


fn signal_action(action: SignedActionHashed) -> ExternResult<()> {
    match action.hashed.content.clone() {
        Action::Create(_create) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                emit_signal(Signal::EntryCreated { action, app_entry })?;
            }
            Ok(())
        }
        Action::Update(update) => {
            if let Ok(Some(app_entry)) = get_entry_for_action(&action.hashed.hash) {
                if let Ok(Some(original_app_entry)) =
                    get_entry_for_action(&update.original_action_address)
                {
                    emit_signal(Signal::EntryUpdated {
                        action,
                        app_entry,
                        original_app_entry,
                    })?;
                }
            }
            Ok(())
        }
        Action::Delete(delete) => {
            if let Ok(Some(original_app_entry)) = get_entry_for_action(&delete.deletes_address) {
                emit_signal(Signal::EntryDeleted {
                    action,
                    original_app_entry,
                })?;
            }
            Ok(())
        }
        Action::CreateLink(create_link) => {
            if let Ok(Some(link_type)) =
                LinkTypes::from_type(create_link.zome_index, create_link.link_type)
            {
                emit_signal(Signal::LinkCreated { action, link_type })?;
            }
            Ok(())
        }
        Action::DeleteLink(delete_link) => {
            let record = get(delete_link.link_add_address.clone(), GetOptions::default())?.ok_or(
                guest_error!(
                    "Failed to fetch CreateLink action".to_string()
                ),
            )?;
            match record.action() {
                Action::CreateLink(create_link) => {
                    if let Ok(Some(link_type)) =
                        LinkTypes::from_type(create_link.zome_index, create_link.link_type)
                    {
                        emit_signal(Signal::LinkDeleted { action, link_type })?;
                    }
                    Ok(())
                }
                _ => {
                    return Err(guest_error!(
                        "Create Link should exist".to_string()
                    ));
                }
            }
        }
        _ => Ok(()),
    }
}


fn get_entry_for_action(action_hash: &ActionHash) -> ExternResult<Option<EntryTypes>> {
    let record = match get_details(action_hash.clone(), GetOptions::default())? {
        Some(Details::Record(record_details)) => record_details.record,
        _ => {
            return Ok(None);
        }
    };
    let entry = match record.entry().as_option() {
        Some(entry) => entry,
        None => {
            return Ok(None);
        }
    };
    let (zome_index, entry_index) = match record.action().entry_type() {
        Some(EntryType::App(AppEntryDef {
            zome_index,
            entry_index,
            ..
        })) => (zome_index, entry_index),
        _ => {
            return Ok(None);
        }
    };
    Ok(EntryTypes::deserialize_from_type(
        zome_index.clone(),
        entry_index.clone(),
        entry,
    )?)
}


#[hdk_extern]
pub fn receive_device_invitation(dia: DeviceInviteAcceptance) -> ExternResult<()> {
    let dia_bytes: SerializedBytes = dia.try_into().map_err(|err| {
        guest_error!(format!(
            "Can't serialize object: {:?}",
            err
        ))
    })?;

    emit_signal(Signal::InvitationReceived {
        device_invite_acceptance: dia_bytes.bytes().to_owned(),
    })?;
    Ok(())
}


#[hdk_extern]
pub fn sign(bytes: serde_bytes::ByteBuf) -> ExternResult<Signature> {
    sign_raw(
        agent_id()?,
        bytes.into_vec(),
    )
}
