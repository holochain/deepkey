use deepkey_integrity::hdk::prelude::*;
use deepkey_integrity::generator::entry::Generator;
use deepkey_integrity::entry::EntryTypes;

#[hdk_extern]
fn new_generator(new_generator: Generator) -> ExternResult<ActionHash> {
    create_entry(EntryTypes::Generator(new_generator))
}
