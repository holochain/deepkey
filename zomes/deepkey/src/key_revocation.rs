use hdi::prelude::*;

use crate::Authorization;


#[hdk_entry_helper]
#[derive(Clone, PartialEq)]
pub struct KeyRevocation {
    pub prior_key_registration: ActionHash,
    pub revocation_authorization: Vec<Authorization>,
}
