use hdk::prelude::*;
use crate::generator::entry::Generator;

#[hdk_extern]
fn new_generator(new_generator: Generator) -> ExternResult<HeaderHash> {
    create_entry(new_generator)
}