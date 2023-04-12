#![warn(warnings)]

use hdk::prelude::*;
// use hdk::prelude::holo_hash::*;
use holochain::conductor::config::ConductorConfig;
use holochain::sweettest::{
    SweetCell, SweetConductor, SweetConductorBatch, SweetDnaFile, SweetZome,
};
use holochain::test_utils::consistency_10s;

use mews_integrity::*;

const DNA_FILEPATH: &str = "../../workdir/clutter.dna";
const ZOME_NAME: &str = "mews";

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
    assert_eq!(recommended_feed.len(), 1);
}
