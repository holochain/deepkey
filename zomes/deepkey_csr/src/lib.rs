//! # Welcome!
//!
//! - Source code - [github.com/holochain/deepkey](https://github.com/holochain/deepkey)
//!
//!
//! ## Usage
//!
//! > **DISCLAIMER:** The real use-case will always be through a client connecting to Holochain's
//! Conductor API.  These examples are to help devs contributing to deepkey understand the rust
//! code, though they may also be useful for understanding the sequence of events.
//!
//! For more information about usage, see the
//! [`README.md`](https://github.com/holochain/deepkey/blob/main/README.md) file in the source code
//! repository.
//!
//! ### Minimal usage
//!
//! 1. [Register a new key](key_registration::create_key)
//!
//! ### Minimal usage with updated change rule
//!
//! 1. [Update change rule to use revocation keys](change_rule::update_change_rule)
//! 2. [Register a new key](key_registration::create_key)
//!
//! ### Full-lifecycle usage
//!
//! 1. [Update change rule to use revocation keys](change_rule::update_change_rule)
//! 2. [Register a new key](key_registration::create_key)
//! 3. [Update a key](key_registration::update_key)
//! 4. [Revoke a key](key_registration::revoke_key)

pub mod change_rule;
pub mod device;
pub mod key_anchor;
pub mod key_registration;
pub mod keyset_root;
pub mod source_of_authority;
pub mod app_binding;
pub mod key_meta;
pub mod utils;

// Re-exports
pub use hdk_extensions;
pub use hdk_extensions::hdi_extensions;
pub use deepkey;
pub use deepkey::deepkey_types;

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
    create_keyset_root()?;

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
pub fn sign(bytes: serde_bytes::ByteBuf) -> ExternResult<Signature> {
    sign_raw(
        agent_id()?,
        bytes.into_vec(),
    )
}


#[hdk_extern]
pub fn query_whole_chain() -> ExternResult<Vec<Record>> {
    Ok(
        query(
            ChainQueryFilter::new()
                // .include_entries(true)
        )?
    )
}
