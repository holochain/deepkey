use deepkey_integrity::*;
use hdk::prelude::*;
#[hdk_extern]
pub fn create_change_rule(change_rule: ChangeRule) -> ExternResult<Record> {
    let change_rule_hash = create_entry(&EntryTypes::ChangeRule(change_rule.clone()))?;
    let record = get(change_rule_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
        WasmErrorInner::Guest(String::from("Could not find the newly created ChangeRule"))
    ))?;
    Ok(record)
}
#[hdk_extern]
pub fn get_change_rule(original_change_rule_hash: ActionHash) -> ExternResult<Option<Record>> {
    let links = get_links(
        original_change_rule_hash.clone(),
        LinkTypes::ChangeRuleUpdates,
        None,
    )?;
    let latest_link = links
        .into_iter()
        .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    let latest_change_rule_hash = match latest_link {
        Some(link) => ActionHash::from(link.target.clone()),
        None => original_change_rule_hash.clone(),
    };
    get(latest_change_rule_hash, GetOptions::default())
}
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateChangeRuleInput {
    pub original_change_rule_hash: ActionHash,
    pub previous_change_rule_hash: ActionHash,
    pub updated_change_rule: ChangeRule,
}
#[hdk_extern]
pub fn update_change_rule(input: UpdateChangeRuleInput) -> ExternResult<Record> {
    let updated_change_rule_hash = update_entry(
        input.previous_change_rule_hash.clone(),
        &input.updated_change_rule,
    )?;
    create_link(
        input.original_change_rule_hash.clone(),
        updated_change_rule_hash.clone(),
        LinkTypes::ChangeRuleUpdates,
        (),
    )?;
    let record =
        get(updated_change_rule_hash.clone(), GetOptions::default())?.ok_or(wasm_error!(
            WasmErrorInner::Guest(String::from("Could not find the newly updated ChangeRule"))
        ))?;
    Ok(record)
}

// use hdk::prelude::*;
// use deepkey_integrity::*;

// #[hdk_extern]
// pub fn create_first_entry(first_entry: FirstEntry) -> ExternResult<Record> {
//   let first_entry_hash = create_entry(&EntryTypes::FirstEntry(first_entry.clone()))?;

//   let record = get(first_entry_hash.clone(), GetOptions::default())?
//         .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly created FirstEntry"))))?;

//   Ok(record)
// }

// #[hdk_extern]
// pub fn get_first_entry(original_first_entry_hash: ActionHash) -> ExternResult<Option<Record>> {
//   let links = get_links(original_first_entry_hash.clone(), LinkTypes::FirstEntryUpdates, None)?;

//   let latest_link = links.into_iter().max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));

//   let latest_first_entry_hash = match latest_link {
//     Some(link) => ActionHash::from(link.target.clone()),
//     None => original_first_entry_hash.clone()
//   };

//   get(latest_first_entry_hash, GetOptions::default())
// }
// #[derive(Serialize, Deserialize, Debug)]
// pub struct UpdateFirstEntryInput {
//   pub original_first_entry_hash: ActionHash,
//   pub previous_first_entry_hash: ActionHash,
//   pub updated_first_entry: FirstEntry
// }

// #[hdk_extern]
// pub fn update_first_entry(input: UpdateFirstEntryInput) -> ExternResult<Record> {
//   let updated_first_entry_hash = update_entry(input.previous_first_entry_hash.clone(), &input.updated_first_entry)?;

//   create_link(input.original_first_entry_hash.clone(), updated_first_entry_hash.clone(), LinkTypes::FirstEntryUpdates, ())?;

//   let record = get(updated_first_entry_hash.clone(), GetOptions::default())?
//         .ok_or(wasm_error!(WasmErrorInner::Guest(String::from("Could not find the newly updated FirstEntry"))))?;

//   Ok(record)
// }
// #[hdk_extern]
// pub fn delete_first_entry(original_first_entry_hash: ActionHash) -> ExternResult<ActionHash> {
//   delete_entry(original_first_entry_hash)
// }
