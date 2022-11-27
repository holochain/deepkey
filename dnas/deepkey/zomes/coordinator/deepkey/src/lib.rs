pub mod change_rule;

use hdk::prelude::*;

#[hdk_extern]
pub fn init(_: ()) -> ExternResult<InitCallbackResult> {
  Ok(InitCallbackResult::Pass)
}
