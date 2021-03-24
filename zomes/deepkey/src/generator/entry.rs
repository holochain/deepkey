use hdk::prelude::*;
use crate::change_rule::entry::Authorization;
use crate::key::entry::Key;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Change {
    new_key: Key,
    authorization: Vec<Authorization>,
}

impl Change {
    pub fn as_new_key_ref(&self) -> &Key {
        &self.new_key
    }

    pub fn as_authorization_ref(&self) -> &[Authorization] {
        &self.authorization
    }
}

#[hdk_entry(id = "generator")]
pub struct Generator {
    change_rule: EntryHash,
    change: Change,
}

impl Generator {
    pub fn as_change_rule_ref(&self) -> &EntryHash {
        &self.change_rule
    }

    pub fn as_change_ref(&self) -> &Change {
        &self.change
    }
}

impl TryFrom<&Element> for Generator {
    type Error = crate::error::Error;
    fn try_from(element: &Element) -> Result<Self, Self::Error> {
        match element.header() {
            // Only creates are allowed for a Generator.
            Header::Create(_) => {
                Ok(match element.entry() {
                    ElementEntry::Present(serialized) => match Self::try_from(serialized) {
                        Ok(deserialized) => deserialized,
                        Err(e) => return Err(crate::error::Error::Wasm(e)),
                    }
                    __ => return Err(crate::error::Error::EntryMissing),
                })
            },
            _ => Err(crate::error::Error::WrongHeader),
        }

    }
}