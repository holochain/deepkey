use hdk::prelude::*;

#[hdk_extern]
fn create_generator(new_generator: Generator) -> ExternResult<HeaderHash> {
    create_entry(new_generator)
}

#[hdk_extern]
fn update_generator((old_generator, new_generator): (HeaderHash, Generator)) -> ExternResult<HeaderHash> {
    update_entry(old_generator, new_generator)
}

#[hdk_extern]
fn delete_generator(old_generator: HeaderHash) -> ExternResult<HeaderHash> {
    delete_entry(old_generator)
}