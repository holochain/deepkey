#![warn(warnings)]

use hdk::prelude::*;
use holochain::conductor::config::ConductorConfig;
use holochain::sweettest::{
    SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile, SweetZome,
};
use holochain::test_utils::consistency_10s;

use deepkey_integrity::*; // for the types

const DNA_FILEPATH: &str = "../../../workdir/deepkey.dna";
const ZOME_NAME: &str = "deepkey";

#[tokio::test(flavor = "multi_thread")]
async fn trusted_feed_is_based_on_follow_topics() {
    let mut agent_group = setup().await;
    let agents = agent_group.create_agents().await;
    let ann = &agents[0];
    let bob = &agents[1];
    let cat = &agents[2];

    // ann.follow(FollowInput {
    //     agent: bob.pubkey.clone(),
    //     follow_topics: vec![FollowTopicInput {
    //         topic: String::from("holochain"),
    //         weight: String::from("1.0"),
    //     }],
    //     follow_other: false,
    // })
    // .await;

    // ann.follow(FollowInput {
    //     agent: cat.pubkey.clone(),
    //     follow_topics: vec![FollowTopicInput {
    //         topic: String::from("blockchain"),
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

    // cat.create_mew(CreateMewInput {
    //     mew_type: MewType::Original,
    //     text: Some(String::from("Doh #holochain when moon?")),
    //     links: None,
    // })
    // .await;

    let key_generation: KeyGeneration = ann.call(
      "instantiate_key_generation",
      bob.pubkey.clone(),
    ).await;

    // The enum Create option from the KeyRegistration options.
    let key_registration = KeyRegistration::Create(key_generation);


    // Register the new key
    let _unit: () = ann.call(
        "new_key_registration",
        key_registration,
    ).await;

    let key_registration_record: Record = ann.call(
        "get_key_registration_from_agent_pubkey_key_anchor",
        bob.pubkey.clone(),
    ).await;

    consistency_10s([
        &(ann.cell.clone()),
        &(bob.cell.clone()),
        &(cat.cell.clone()),
    ])
    .await;
}

// #[tokio::test(flavor = "multi_thread")]
// async fn trusted_feed_is_ordered_by_topic_weights_and_recency() {
//     let mut agent_group = setup().await;
//     let agents = agent_group.create_agents().await;
//     let ann = &agents[0];
//     let bob = &agents[1];

//     ann.follow(FollowInput {
//         agent: bob.pubkey.clone(),
//         follow_topics: vec![FollowTopicInput {
//             topic: String::from("holochain"),
//             weight: String::from("0.9"),
//         }],
//         follow_other: false,
//     })
//     .await;

//     bob.create_mew(CreateMewInput {
//         mew_type: MewType::Original,
//         text: Some(String::from("OLD #holochain mew")),
//         links: None,
//     })
//     .await;

//     let oldest_mew_seconds = 2;
//     std::thread::sleep(std::time::Duration::from_secs(oldest_mew_seconds));

//     bob.create_mew(CreateMewInput {
//         mew_type: MewType::Original,
//         text: Some(String::from("NEW #holochain mew")),
//         links: None,
//     })
//     .await;

//     consistency_10s([&(ann.cell.clone()), &(bob.cell.clone())]).await;

//     let recommended_feed = ann
//         .recommended(RecommendedInput {
//             now: Timestamp::now(),
//             oldest_mew_seconds: Some(oldest_mew_seconds),
//         })
//         .await;

//     assert_eq!(recommended_feed.len(), 1);
//     assert_eq!(
//         recommended_feed[0].mew.content.as_ref().unwrap().text,
//         String::from("NEW #holochain mew")
//     );
// }

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

    // pub async fn create_mew(&self, input: CreateMewInput) -> ActionHash {
    //     self.conductor.call(&self.zome, "create_mew", input).await
    // }

    // pub async fn recommended(&self, input: RecommendedInput) -> Vec<FeedMew> {
    //     self.conductor.call(&self.zome, "recommended", input).await
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
