use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::generator::entry::Generator;

#[hdk_extern]
fn new_generator(new_generator: Generator) -> ExternResult<HeaderHash> {
    create_entry(new_generator)
}
