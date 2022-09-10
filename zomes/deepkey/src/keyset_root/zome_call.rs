use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::keyset_root::entry::KeysetRoot;
use deepkey_integrity::change_rule::entry::{ AuthorizedSpecChange, ChangeRule };
use deepkey_integrity::entry::EntryTypes;

/// 
/// create_keyset_root -- Create an initial KeysetRoot, or an switch to a new KeysetRoot w/ proper authority
/// 
///     When initializing a Deepkey, the ChangeRule{ keyset_root, keyset_leaf, ... } will reference
/// the KeysetRoot just committed.  Otherwise, the provided keyset_root indicates that we are
/// switching from an old KeysetRoot to a new one.
/// 
///     If the validation for the ChangeRule fails (improper authority provided), the commit of the
/// new KeysetRoot is rolled back.
/// 
#[hdk_extern]
fn create_keyset_root((new_keyset_root, spec_change, keyset_root): (KeysetRoot, AuthorizedSpecChange, Option<ActionHash>)) -> ExternResult<(ActionHash, ActionHash)> {
    let keyset_leaf = create_entry(EntryTypes::KeysetRoot(new_keyset_root))?;
    let change_rule = match keyset_root {
        // If we're initializing Deepkey w/ its first KeysetRoot
        None => create_entry(EntryTypes::ChangeRule(ChangeRule{
            keyset_leaf: keyset_leaf.clone(), keyset_root: keyset_leaf.clone(), spec_change
        }))?,
        // If we're updating an existing keyset_root with a new KeysetRoot
        Some(keyset_root) => create_entry(EntryTypes::ChangeRule(ChangeRule{
            keyset_leaf: keyset_leaf.clone(), keyset_root, spec_change
        }))?,
    };
    Ok((keyset_leaf, change_rule))
}
