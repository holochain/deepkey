#![warn(warnings)]

use hdk::prelude::*;
use holochain::conductor::config::ConductorConfig;
use holochain::sweettest::{
    SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile, SweetZome,
};
use holochain::test_utils::consistency_10s;
use serial_test::serial;
use deepkey_integrity::*; // for the types

const DNA_FILEPATH: &str = "../../../workdir/deepkey.dna";
const ZOME_NAME: &str = "deepkey";

#[serial]
#[tokio::test(flavor = "multi_thread")]
async fn revoke_key_registration() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];

    let key_generation: KeyGeneration = ann
        .call("instantiate_key_generation", ann.pubkey.clone())
        .await;
    let key_registration_hash: ActionHash = ann
        .call(
            "new_key_registration",
            KeyRegistration::Create(key_generation),
        )
        .await;
    let key_revocation1 = KeyRevocation {
        prior_key_registration: key_registration_hash,
        revocation_authorization: Vec::new(),
    };
    let key_revocation2: KeyRevocation =
        ann.call("authorize_key_revocation", key_revocation1).await;

    consistency_10s([&(ann.cell.clone())]).await;

    // Query KeyAnchor; should return the key_registration
    let key_registration_record: Record = ann
        .call("get_key_registration_from_key_anchor", ann.pubkey.clone())
        .await;
    println!("key_registration_record: {:?}", key_registration_record);

    // let key_registration_hash2: ActionHash = ann
    //     .call(
    //         "new_key_registration",
    //         KeyRegistration::Delete(key_revocation2),
    //     )
    //     .await;
    // // Query via KeyAnchor; should return nothing
    // let key_registration_record: Record = ann
    //     .call("get_key_registration_from_key_anchor", ann.pubkey.clone())
    //     .await;
    // println!(
    //     "revoked key_registration_record: {:?}",
    //     key_registration_record
    // );
}

#[serial]
#[tokio::test(flavor = "multi_thread")]
async fn create_and_authorize_key_revocation() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];
    // let bob = &agents[1];
    // let cat = &agents[2];

    // TODO: Install a second test DNA and register that pubkey
    let key_generation: KeyGeneration = ann
        .call("instantiate_key_generation", ann.pubkey.clone())
        .await;

    // Register the new key
    let key_registration_hash: ActionHash = ann
        .call(
            "new_key_registration",
            KeyRegistration::Create(key_generation),
        )
        .await;

    let key_revocation1 = KeyRevocation {
        prior_key_registration: key_registration_hash,
        revocation_authorization: Vec::new(),
    };

    // Ann authorizes the revocation
    let key_revocation2: KeyRevocation =
        ann.call("authorize_key_revocation", key_revocation1).await;

    consistency_10s([&(ann.cell.clone())]).await;
    assert_eq!(key_revocation2.revocation_authorization.len(), 1);
}

#[serial]
#[tokio::test(flavor = "multi_thread")]
async fn create_and_retrieve_key_registration() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];
    // let bob = &agents[1];
    // let cat = &agents[2];

    let key_generation: KeyGeneration = ann
        .call("instantiate_key_generation", ann.pubkey.clone())
        .await;

    let key_registration = KeyRegistration::Create(key_generation);

    // Register the new key
    let _hash: ActionHash = ann
        .call("new_key_registration", key_registration.clone())
        .await;

    let key_registration_record: Record = ann
        .call("get_key_registration_from_key_anchor", ann.pubkey.clone())
        .await;

    let key_registration2: KeyRegistration = key_registration_record
        .entry
        .to_app_option::<KeyRegistration>()
        .unwrap()
        .unwrap();

    consistency_10s([&(ann.cell.clone())]).await;

    assert_eq!(key_registration, key_registration2);
}

// TEST HELPERS:

pub struct Agent<'a> {
    pub cell: SweetCell,
    pub conductor: &'a SweetConductor,
    pub pubkey: AgentPubKey,
    pub zome: SweetZome,
}

impl Agent<'_> {
    pub async fn call<I, O>(&self, fn_name: &str, input: I) -> O
    where
        I: serde::Serialize + std::fmt::Debug,
        O: serde::de::DeserializeOwned + std::fmt::Debug,
    {
        call_zome(&self, fn_name, input).await
    }

    // pub async fn follow(&self, input: FollowInput) -> () {
    //     self.conductor.call(&self.zome, "follow", input).await
    // }
}

pub struct AgentGroup {
    conductors: SweetConductorBatch,
}

impl AgentGroup {
    pub async fn create_agents<'a>(&'a mut self) -> Vec<Agent<'a>> {
        let dna_path = std::env::current_dir().unwrap().join(DNA_FILEPATH);
        let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

        let apps = self.conductors.setup_app(ZOME_NAME, &[dna]).await.unwrap();
        self.conductors.exchange_peer_info().await;

        let ((ann_cell,), (bob_cell,), (cat_cell,)) = apps.into_tuples();

        let ann = Agent {
            cell: ann_cell.clone(),
            conductor: self.conductors.get(0).unwrap(),
            pubkey: ann_cell.agent_pubkey().clone(),
            zome: ann_cell.zome(ZOME_NAME),
        };
        let bob = Agent {
            cell: bob_cell.clone(),
            conductor: self.conductors.get(1).unwrap(),
            pubkey: bob_cell.agent_pubkey().clone(),
            zome: bob_cell.zome(ZOME_NAME),
        };
        let cat = Agent {
            cell: cat_cell.clone(),
            conductor: self.conductors.get(2).unwrap(),
            pubkey: cat_cell.agent_pubkey().clone(),
            zome: cat_cell.zome(ZOME_NAME),
        };

        vec![ann, bob, cat]
    }
}

pub async fn setup() -> AgentGroup {
    let conductors = SweetConductorBatch::from_config(3, ConductorConfig::default()).await;
    AgentGroup { conductors }
}

pub async fn call_zome<I, O>(agent: &Agent<'_>, fn_name: &str, payload: I) -> O
where
    I: serde::Serialize + std::fmt::Debug,
    O: serde::de::DeserializeOwned + std::fmt::Debug,
{
    agent.conductor.call(&agent.zome, fn_name, payload).await
}

// Examples:

// consistency_10s([
//     &(ann.cell.clone()),
//     &(bob.cell.clone()),
//     &(cat.cell.clone()),
// ])
// .await;

// ann.follow(FollowInput {
//     agent: bob.pubkey.clone(),
//     follow_topics: vec![FollowTopicInput {
//         topic: String::from("holochain"),
//         weight: String::from("1.0"),
//     }],
//     follow_other: false,
// })
// .await;

// bob.create_mew(CreateMewInput {
//     mew_type: MewType::Original,
//     text: Some(String::from("Wow #holochain is cool!")),
//     links: None,
// })
// .await;
