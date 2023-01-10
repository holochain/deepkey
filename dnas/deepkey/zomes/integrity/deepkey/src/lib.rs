pub mod change_rule;
pub mod device_invite;
pub mod device_invite_acceptance;
pub mod keyset_root;

use change_rule::*;
use device_invite::DeviceInvite;
use device_invite_acceptance::DeviceInviteAcceptance;
use hdi::prelude::*;
use keyset_root::*;
// use device_invite::*;

// @todo - e.g. configurable difficulty over hashing the DNA - https://docs.rs/pow/0.2.0/pow/
#[hdk_entry_helper]
pub struct ProofOfWork([u8; 32]);

// @todo
#[hdk_entry_helper]
pub struct ProofOfStake([u8; 32]);

// @todo
#[hdk_entry_helper]
pub struct ProofOfAuthority([u8; 32]);

#[hdk_entry_helper]
pub enum MembraneProof {
    // No additional membrane.
    None,
    // Proof of Work membrane.
    ProofOfWork(ProofOfWork),
    // Proof of Stake membrane.
    ProofOfStake(ProofOfStake),
    // Proof of Authority membrane.
    ProofOfAuthority(ProofOfAuthority),
}

#[hdk_entry_helper]
pub enum KeysetProof {
    KeysetRoot(KeysetRoot),
    // TODO: invitation
    // DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_entry_helper]
pub struct JoiningProof {
    keyset_proof: KeysetProof,
    membrane_proof: MembraneProof,
}

impl JoiningProof {
    pub fn new(keyset_proof: KeysetProof, membrane_proof: MembraneProof) -> Self {
        Self {
            keyset_proof,
            membrane_proof,
        }
    }
}

#[hdk_entry_defs]
#[unit_enum(UnitEntryTypes)]
pub enum EntryTypes {
    #[entry_def]
    JoiningProof(JoiningProof),
    #[entry_def]
    KeysetRoot(KeysetRoot),
    #[entry_def]
    ChangeRule(ChangeRule),
    #[entry_def]
    DeviceInvite(DeviceInvite),
    #[entry_def]
    DeviceInviteAcceptance(DeviceInviteAcceptance),
}

#[hdk_link_types]
pub enum LinkTypes {
    AgentToMembraneProof,
    AgentPubKeyToDeviceInvite
}

////////////////////////////////////////////////////////////////////////////////
// Genesis self-check callback
////////////////////////////////////////////////////////////////////////////////
// #[hdk_extern]
// pub fn genesis_self_check(data: GenesisSelfCheckData) -> ExternResult<ValidateCallbackResult> {
//     is_membrane_proof_valid(data.agent_key, data.membrane_proof)
// }

////////////////////////////////////////////////////////////////////////////////
// Validation callback
////////////////////////////////////////////////////////////////////////////////

pub fn validate(_op: Op) -> ExternResult<ValidateCallbackResult> {
    return Ok(ValidateCallbackResult::Valid);
    // let op_type: OpType<EntryTypes, ()> = ;
    // match op.to_type::<EntryTypes, LinkTypes>()? {
    //     OpType::StoreRecord(_) => todo!(),
    //     OpType::StoreEntry(_) => todo!(),
    //     OpType::RegisterAgentActivity(_) => todo!(),
    //     OpType::RegisterCreateLink {
    //         base_address,
    //         target_address,
    //         tag,
    //         link_type,
    //     } => todo!(),
    //     OpType::RegisterDeleteLink {
    //         original_link_hash,
    //         base_address,
    //         target_address,
    //         tag,
    //         link_type,
    //     } => todo!(),
    //     OpType::RegisterUpdate(_) => todo!(),
    //     OpType::RegisterDelete(_) => todo!(),
    // }

    /*
    match op {
        // Validation for entries
        Op::StoreEntry {
            action:
                SignedHashed {
                    hashed: HoloHashed {
                        content: action, ..
                    },
                    ..
                },
            entry,
        } => {
            if let Some(AppEntryType {
                id: entry_def_index,
                zome_id,
                ..
            }) = action.app_entry_type()
            {
                match EntryTypes::deserialize_from_type(*zome_id, *entry_def_index, &entry)? {
                    Some(EntryTypes::MyThing1(_my_thing1)) => (),
                    Some(EntryTypes::MyThing2(_my_thing2)) => (),
                    Some(EntryTypes::MyThingPrivate(_my_thing_private)) => (),
                    None => return Ok(ValidateCallbackResult::Invalid(
                        "expected app entry type, got none".to_string(),
                    )),
                }
            }
        },
        Op::RegisterUpdate { .. } => return Ok(ValidateCallbackResult::Invalid(
            "updating entries isn't valid".to_string(),
        )),
        Op::RegisterDelete { .. } => return Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),

        // Validation for links
        Op::RegisterCreateLink { create_link } => {
            let (create_link, _) = create_link.hashed.into_inner();
            match create_link.link_type.into() {
                LinkTypes::Fish => (),
                LinkTypes::Dog => (),
                LinkTypes::Cow => (),
            }
        },
        Op::RegisterDeleteLink { delete_link: _, create_link} => {
            match create_link.link_type.into() {
                LinkTypes::Fish => (),
                LinkTypes::Dog => (),
                LinkTypes::Cow => (),
            }
        },

        // Validation for records based on action type
        Op::StoreRecord { record } => {
            match record.action() {
                // Validate agent joining the network
                Action::AgentValidationPkg(_) => todo!(),

                // Validate entries
                Action::Create(create) => match create.entry_type {
                    EntryTypes::MyThing1(_) => todo!(),
                    EntryTypes::MyThing2(_) => todo!(),
                    EntryTypes::MyThingPrivate(_) => todo!(),
                },
                Action::Update(_) => todo!(),
                Action::Delete(_) => todo!(),

                // Validate Links
                Action::CreateLink(_) => todo!(),
                Action::DeleteLink(_) => todo!(),

                // Validation chain migration
                Action::OpenChain(_) => todo!(),
                Action::CloseChain(_) => todo!(),

                // Validate capabilities, rarely used
                Action::CapGrant() => todo!(),
                Action::CapClaim() => todo!(),

                // Validate init and genesis entries, also rarely
                Action::InitZomesComplete(_)=>todo!(),
                Action::AgentValidationPkg(_)=>todo!(), // mostly this will be validated in the process of using it to validate the Agent Key
                Action::Dna(_)=>todo!(),
            };
        },

        // Agent joining network validation
        // this is a new DHT op
        Op::RegisterAgent { action, agent_pub_key } => {
            // get validation package and then do stuff
         //   Ok(ValidateCallbackResult::Valid)
        },
        // Chain structure validation
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }

    // this is what we currently have to do to make things work
    let info = zome_info()?;
    match op {
        Op::StoreRecord { record } => {
            match record.action() {
                Action::Dna(_) => todo!(),
                Action::AgentValidationPkg(_) => todo!(),
                Action::InitZomesComplete(_) => todo!(),
                Action::CreateLink(create) => match create.link_type.into() {
                    LinkTypes::Fish => todo!(),
                    _ => {}
                },
                Action::DeleteLink(_) => todo!(),
                Action::OpenChain(_) => todo!(),
                Action::CloseChain(_) => todo!(),
                Action::Create(create) => match create.entry_type {
                    EntryType::AgentPubKey => todo!(),
                    EntryType::App(app_entry_type) => {
                        match info.entry_defs.get(app_entry_type.id.index()).map(|entry_def| entry_def.id.to_string()) {
                            "my_entry1" => _
                        }
                    }
                    EntryType::CapClaim => todo!(),
                    EntryType::CapGrant => todo!(),
                },
                Action::Update(_) => todo!(),
                Action::Delete(_) => todo!(),
            }
            Ok(ValidateCallbackResult::Valid)
        }
        Op::StoreEntry { action, .. } => {
            match action.hashed.content.entry_type() {
                entry_def_index!(String::from("my_entry1")) => todo!(),
                _ => {}
            }
            Ok(ValidateCallbackResult::Valid)
        }
        Op::RegisterCreateLink { create_link: _ } => Ok(ValidateCallbackResult::Valid),
        Op::RegisterDeleteLink { create_link: _, .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting links isn't valid".to_string(),
        )),
        Op::RegisterUpdate { .. } => Ok(ValidateCallbackResult::Invalid(
            "updating entries isn't valid".to_string(),
        )),
        Op::RegisterDelete { .. } => Ok(ValidateCallbackResult::Invalid(
            "deleting entries isn't valid".to_string(),
        )),
        Op::RegisterAgentActivity { .. } => Ok(ValidateCallbackResult::Valid),
    }
     */
}
