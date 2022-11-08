use hdi::prelude::*;
// use hdk::prelude::*;
// use crate::keyset_root::entry::KeysetRoot;
// use crate::keyset_root::entry::KEYSET_ROOT_CHAIN_INDEX;
// use crate::keyset_root::error::Error;
use std::u8;

////////////////////////////////////////////////////////////////////////////////
// Entry declarations
////////////////////////////////////////////////////////////////////////////////

////////////////////////////////////////////////////////////////////////////////
// Entry struct definitions with necessary impls
pub const KEYSET_ROOT_INDEX: EntryDefIndex = EntryDefIndex(3);
/// KeysetRoot must be the 4th entry on `FirstDeepkeyAgent`'s chain.
pub const KEYSET_ROOT_CHAIN_INDEX: u32 = 3;

// #[hdk_entry(id = "keyset_root")]
/// We need an entry to create a permanent anchor that can be used to reference the space of keys under the control of a human agent.
/// This is commited only by the FirstDeepkeyAgent (FDA) not later devices that are joining this same agency context.

// #[derive(Clone)]
#[hdk_entry_helper]
pub struct KeysetRoot {
    pub first_deepkey_agent: AgentPubKey,
    /// The private key is thrown away.
    root_pub_key: AgentPubKey,
    fda_pubkey_signed_by_root_key: Signature,
}


/*
#[hdk_entry_helper]
pub struct MyThing1 {
    pub thing1: String,
}
#[hdk_entry_helper]
pub struct MyThing2 {
    pub thing2: String,
}
impl MyThing2 {
    pub fn some_fn() {
        debug!("Do something")
    }
}
#[hdk_entry_helper]
pub struct MyThingPrivate {
    pub private_thing: String,
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def(required_validations = 5)]
    MyThing1(MyThing1), 
    #[entry_def(required_validations = 5)]
    MyThing2(MyThing2),
    #[entry_def(required_validations = 5, visibility = "private")]
    MyThingPrivate(MyThingPrivate),
}
*/
