#![warn(warnings)]

use hdk::prelude::*;
use holochain::conductor::config::ConductorConfig;
use holochain::sweettest::{
    SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile, SweetZome,
};
use holochain::test_utils::consistency_10s;

use mews_integrity::*;

const DNA_FILEPATH: &str = "../../workdir/deepkey.dna";
const ZOME_NAME: &str = "deepkey";

pub struct Agent<'a> {
    pub cell: SweetCell,
    pub conductor: &'a SweetConductor,
    pub pubkey: AgentPubKey,
    pub zome: SweetZome,
}

pub async fn call_zome<I, O>(agent: &Agent<'_>, fn_name: &str, payload: I) -> O
where
    I: serde::Serialize + std::fmt::Debug,
    O: serde::de::DeserializeOwned + std::fmt::Debug,
{
    agent.conductor.call(&agent.zome, fn_name, payload).await
}

#[tokio::test(flavor = "multi_thread")]
async fn trusted_feed_is_based_on_follow_topics() {
    // Use prebuilt DNA file
    let dna_path = std::env::current_dir().unwrap().join(DNA_FILEPATH);
    let dna = SweetDnaFile::from_bundle(&dna_path).await.unwrap();

    // Set up conductors
    let mut conductors = SweetConductorBatch::from_config(3, ConductorConfig::default()).await;
    let apps = conductors.setup_app(ZOME_NAME, &[dna]).await.unwrap();
    conductors.exchange_peer_info().await;

    let ((ann_cell,), (bob_cell,), (cat_cell,)) = apps.into_tuples();

    let ann = Agent {
        cell: ann_cell.clone(),
        conductor: conductors.get(0).unwrap(),
        pubkey: ann_cell.agent_pubkey().clone(),
        zome: ann_cell.zome(ZOME_NAME),
    };
    let bob = Agent {
        cell: bob_cell.clone(),
        conductor: conductors.get(1).unwrap(),
        pubkey: bob_cell.agent_pubkey().clone(),
        zome: bob_cell.zome(ZOME_NAME),
    };

    // let _: ActionHash = call_zome(&ann, "fn name", SomeInput {}).await;

    let keyGeneration: KeyGeneration = call_zome(
        &ann,
        "instantiate_key_generation",
        bob.pubkey.clone(),
      );

    // The enum Create option from the KeyRegistration options.
    let keyRegistration = KeyRegistration {
        Create: {
          keyGeneration,
        },
      }

      // Register the new key
      await deepkeyZomeCall(alice)<null>("new_key_registration", keyRegistration)

      consistency_10s([
          &(ann.cell.clone()),
          // &(bob.cell.clone()),
          // &(cat.cell.clone()),
          ])
          .await;

    //   expect(sample).toEqual(
    //     decode((createReadOutput.entry as any).Present.entry) as any
    //   )
    // let recommended_feed: Vec<FeedMew> = call_zome(
    //     &ann,
    //     "recommended",
    //     RecommendedInput {
    //         now: Timestamp::now(),
    //         oldest_mew_seconds: Some(oldest_mew_seconds),
    //     },
    // )
    // .await;

    // assert_eq!(recommended_feed.len(), 1);
    // assert_eq!(
    //     recommended_feed[0].mew.content.as_ref().unwrap().text,
    //     String::from("NEW #holochain mew")
    // );
}
