use crate::key;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
#[serde(transparent)]
struct KeyAnchor(key::PubKey);