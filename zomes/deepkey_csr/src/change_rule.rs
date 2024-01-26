use crate::utils;
use serde_bytes;
use deepkey::*;
use hdk::prelude::*;
use hdk_extensions::{
    agent_id,
    must_get,
    hdi_extensions::{
        guest_error,
        ScopedTypeConnector,
    },
};


#[hdk_extern]
pub fn create_change_rule(change_rule: ChangeRule) -> ExternResult<ActionHash> {
    let create_addr = create_entry( &change_rule.to_input() )?;

    create_link(
        change_rule.keyset_root.clone(),
        create_addr.clone(),
        LinkTypes::KSRToChangeRule,
        (),
    )?;

    Ok( create_addr )
}


#[hdk_extern]
pub fn get_ksr_change_rule_links(ksr_addr: ActionHash) -> ExternResult<Vec<Link>> {
    get_links(
        GetLinksInputBuilder::try_new(
            ksr_addr,
            LinkTypes::KSRToChangeRule,
        )?.build()
    )
}


#[hdk_extern]
pub fn get_current_change_rule_for_ksr(ksr_addr: ActionHash) -> ExternResult<ChangeRule> {
    let change_rule_links = get_ksr_change_rule_links( ksr_addr.clone() )?;
    let latest_addr = change_rule_links
        .iter()
        .filter_map( |link| Some(
            (
                link.timestamp,
                link.target.to_owned().into_any_dht_hash()?
            )
        ))
        .max_by_key( |(timestamp, _)| timestamp.to_owned() )
        .ok_or(guest_error!(format!("There are no change rules for KSR ({})", ksr_addr )))?
        .1;

    Ok( must_get( &latest_addr )?.try_into()? )
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuthoritySpecInput {
    pub sigs_required: u8,
    pub authorized_signers: Vec<serde_bytes::ByteArray<32>>,
}

impl From<AuthoritySpecInput> for AuthoritySpec {
    fn from(input: AuthoritySpecInput) -> Self {
        Self {
            sigs_required: input.sigs_required,
            authorized_signers: input.authorized_signers.iter()
                .map(|key| key.into_array() )
                .collect(),
        }
    }
}


#[hdk_extern]
pub fn construct_authority_spec(input: AuthoritySpecInput) -> ExternResult<(AuthoritySpec, Vec<u8>)> {
    let authority_spec = AuthoritySpec::from( input );
    let serialized = utils::serialize( &authority_spec )?;

    Ok((
        authority_spec,
        serialized,
    ))
}


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct UpdateChangeRuleInput {
    pub authority_spec: AuthoritySpecInput,
    pub authorizations: Option<Vec<Authorization>>,
}

#[hdk_extern]
pub fn update_change_rule(input: UpdateChangeRuleInput) -> ExternResult<ChangeRule> {
    let new_authority_spec = AuthoritySpec::from( input.authority_spec );
    let authorizations = match input.authorizations {
        Some(authorizations) => authorizations,
        None => {
            let fda = agent_id()?;
            debug!("Signing new authority spec with FDA ({})", fda );
            let fda_signature = sign_raw(
                fda,
                utils::serialize( &new_authority_spec )?
            )?;
            vec![ (0, fda_signature) ]
        }
    };
    let spec_change = AuthorizedSpecChange::new(
        new_authority_spec,
        authorizations,
    );

    let latest_change_rule_record = utils::query_entry_type_latest( EntryTypesUnit::ChangeRule )?
        .ok_or(guest_error!(format!(
            "There is no change rule to update"
        )))?;

    ChangeRule::try_from( latest_change_rule_record.clone() )?;

    let keyset_root_addr = utils::query_keyset_root_addr()?;
    let new_change_rule = ChangeRule::new(
        keyset_root_addr.clone(),
        keyset_root_addr.clone(),
        spec_change,
    );

    let update_addr = update_entry(
        latest_change_rule_record.action_address().to_owned(),
        &new_change_rule,
    )?;

    create_link(
        keyset_root_addr.clone(),
        update_addr,
        LinkTypes::KSRToChangeRule,
        (),
    )?;

    Ok( new_change_rule )
}
