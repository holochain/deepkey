use hdk::prelude::*;
use crate::change_rule::entry::Authorization;
#[cfg(test)]
use ::fixt::prelude::*;
#[cfg(test)]
use crate::change_rule::entry::AuthorizationVecFixturator;

/// Same as entry_def_index! but constant.
/// Has test coverage in case entry_defs! ever changes.
pub const GENERATOR_INDEX: EntryDefIndex = EntryDefIndex(5);

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Change {
    new_key: AgentPubKey,
    authorization: Vec<Authorization>,
}

impl Change {
    pub fn new(new_key: AgentPubKey, authorization: Vec<Authorization>) -> Self {
        Self{ new_key, authorization }
    }

    pub fn as_new_key_ref(&self) -> &AgentPubKey {
        &self.new_key
    }

    pub fn as_authorization_ref(&self) -> &[Authorization] {
        &self.authorization
    }
}

#[cfg(test)]
fixturator!(
    Change;
    constructor fn new(AgentPubKey, AuthorizationVec);
);

//#[hdk_entry(id = "generator")]
#[hdk_entry_helper]
#[derive(Clone)]
pub struct Generator {
    change_rule: HeaderHash,
    change: Change,
}

#[cfg(test)]
fixturator!(
    Generator;
    constructor fn new(HeaderHash, Change);
);

impl Generator {
    pub fn new(change_rule: HeaderHash, change: Change) -> Self {
        Self { change_rule, change }
    }

    pub fn as_change_rule_ref(&self) -> &HeaderHash {
        &self.change_rule
    }

    pub fn as_change_ref(&self) -> &Change {
        &self.change
    }
}
/*
 * TODO: How do we limit to Create only?
 * 
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
 */
#[cfg(test)]
pub mod tests {
    use hdk::prelude::*;
    use super::GENERATOR_INDEX;
    use super::Generator;

    #[test]
    fn generator_index_test() {
        assert_eq!(
            GENERATOR_INDEX,
            entry_def_index!(Generator).unwrap()
        )
    }
}
