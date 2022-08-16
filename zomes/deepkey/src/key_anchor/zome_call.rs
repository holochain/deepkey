use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::key_anchor::entry::KeyAnchor;

#[derive(Debug, Serialize, Deserialize)]
pub enum KeyState {
    // Key anchor.
    Valid(SignedHeaderHashed),
    Invalidated(SignedHeaderHashed),
    NotFound,
}

#[hdk_extern]
// Pass in now for the timestamp if you want if currently valid, maybe a little bit in the past for safety.
// This is not about the device or keyset root, this is about the registered and revoked keys in the system
// so this is a key::PubKey
// @todo make timestamp work
fn key_state((key, _timestamp): (KeyAnchor, Timestamp)) -> ExternResult<KeyState> {
    Ok(match get_details(hash_entry(key)?, GetOptions::latest())? {
        Some(details) => {
            match details {
                Details::Entry(entry_details) => {
                    // If update or delete return oldest update or delete
                    // @todo does it need to be the oldest update-or-delete or just oldest update or oldest delete?
                    if entry_details.updates.len() > 0 {
                        KeyState::Invalidated(entry_details.updates[0].clone())
                    }
                    // A delete exists.
                    else if entry_details.deletes.len() > 0 {
                        KeyState::Invalidated(entry_details.deletes[0].clone())
                    }
                    // No updates or deletes so this create is still valid.
                    else if entry_details.headers.len() > 0 {
                        KeyState::Valid(entry_details.headers[0].clone())
                    }
                    // Maybe some rejected headers popped up or something...
                    // No valid CRUD headers at this point though.
                    else {
                        KeyState::NotFound
                    }
                },
                // Holochain returned element details for an entry get!
                _ => KeyState::NotFound,
            }
        },
        // Nothing found.
        None => KeyState::NotFound,
    })
}
