use deepkey::*;
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
pub fn get_change_rule(_original_change_rule_hash: ActionHash) -> ExternResult<Option<Record>> {
    // let links = get_links(
    //     original_change_rule_hash.clone(),
    //     LinkTypes::ChangeRuleUpdates,
    //     None,
    // )?;
    // let latest_link = links
    //     .into_iter()
    //     .max_by(|link_a, link_b| link_b.timestamp.cmp(&link_a.timestamp));
    // let latest_change_rule_hash = match latest_link {
    //     Some(link) => ActionHash::from(link.target.clone()),
    //     None => original_change_rule_hash.clone(),
    // };
    // get(latest_change_rule_hash, GetOptions::default())
    Ok(None)
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
