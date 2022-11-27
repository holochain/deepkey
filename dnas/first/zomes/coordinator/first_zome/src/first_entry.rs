use hdk::prelude::*;
use first_zome_integrity::*;

#[hdk_extern]
pub fn create_first_entry(first_entry: FirstEntry) -> ExternResult<Record> {
  let first_entry_hash = create_entry(&EntryTypes::FirstEntry(first_entry.clone()))?;

    
  let record = get(first_entry_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly created FirstEntry"))))?;

  Ok(record)
}

#[hdk_extern]
pub fn get_first_entry(original_first_entry_hash: ActionHash) -> ExternResult<Option<Record>> {
  let links = get_links(original_first_entry_hash.clone(), LinkTypes::FirstEntryUpdates, None)?;

  let latest_link = links.into_iter().max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
  
  let latest_first_entry_hash = match latest_link {
    Some(link) => ActionHash::from(link.target.clone()),
    None => original_first_entry_hash.clone()   
  };
 
  get(latest_first_entry_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateFirstEntryInput {
  pub original_first_entry_hash: ActionHash,
  pub previous_first_entry_hash: ActionHash,
  pub updated_first_entry: FirstEntry
}

#[hdk_extern]
pub fn update_first_entry(input: UpdateFirstEntryInput) -> ExternResult<Record> {
  let updated_first_entry_hash = update_entry(input.previous_first_entry_hash.clone(), &input.updated_first_entry)?;
        
  create_link(input.original_first_entry_hash.clone(), updated_first_entry_hash.clone(), LinkTypes::FirstEntryUpdates, ())?;

  let record = get(updated_first_entry_hash.clone(), GetOptions::default())?
        .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated FirstEntry"))))?;
    
  Ok(record)
}
#[hdk_extern]
pub fn delete_first_entry(original_first_entry_hash: ActionHash) -> ExternResult<ActionHash> {
  delete_entry(original_first_entry_hash)
}
