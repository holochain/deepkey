use hdk::prelude::*;
use crate::keyset_root::entry::KeysetRoot;
use crate::device_authorization::entry::DeviceAuthorization;

// A deepkey agent is any agent that is either the First Deepkey Agent or anything referenced by a device handshake.
// Either the FDA or anything referenced by a device handshake.

pub enum DeepKeyAgent {
    Root(KeysetRoot),
    Device(DeviceAuthorization),
}

/// Try not to do this because it is not efficient to deserialize if you know the type.
impl TryFrom<&Entry> for DeepKeyAgent {
    type Error = Error;
    fn try_from(entry: &Entry) -> Result<Self, Self::Error> {
        match KeysetRoot::try_from(entry) {
            Ok(keyset_root) => Ok(DeepKeyAgent::Root(keyset_root)),
            Err(_) => match DeviceAuthorization::try_from(entry) {
                Ok(device_authorization) => Ok(DeepKeyAgent::Device(device_authorization)),
                Err(_) => Err(crate::error::Error::EntryNotDeepKeyAgent),
            }
        }
    }
}

impl DeepKeyAgent {
    pub fn is_agent(&self, other: &AgentPubKey) -> bool {
        match self {
            DeepKeyAgent::Root(keyset_root) => keyset_root.as_first_deepkey_agent_ref() == other,
            DeepKeyAgent::Device(device_authorization) => {
                ( device_authorization.as_root_acceptance_ref().0 == *other ) || ( device_authorization.as_device_acceptance_ref().0 == *other )
            }
        }
    }
}