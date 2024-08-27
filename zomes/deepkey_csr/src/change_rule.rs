use crate::{
    utils,
    deepkey_sdk,
};
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
pub use deepkey_sdk::{
    AuthoritySpecInput,
    UpdateChangeRuleInput,
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


#[hdk_extern]
pub fn construct_authority_spec(input: AuthoritySpecInput) -> ExternResult<(AuthoritySpec, Vec<u8>)> {
    let authority_spec = AuthoritySpec::from( input );
    let serialized = utils::serialize( &authority_spec )?;

    Ok((
        authority_spec,
        serialized,
    ))
}


/// Update the rules for updating keys and the rules themselves
///
/// A `ChangeRule` is created in the init process with the cell agent as the authorized signer.
/// This means that the first change rule update can be authorized by the coordinator zome who has
/// the ability to sign with the cell agent.
///
/// #### Example usage (first update)
/// ```rust, no_run
/// use hdk::prelude::*;
/// use hc_deepkey_sdk::*;
///
/// use rand::rngs::OsRng;
/// use ed25519_dalek::SigningKey;
/// use serde_bytes::ByteArray;
/// # fn main() -> ExternResult<()> {
///
/// // Generate some revocation keys (unsafely)
/// let rev_key1 = SigningKey::generate(&mut OsRng);
/// let rev_key2 = SigningKey::generate(&mut OsRng);
///
/// let rev_pubkey1 = rev_key1.verifying_key();
/// let rev_pubkey2 = rev_key2.verifying_key();
///
/// let rev_auth1 = ByteArray::<32>::new( rev_pubkey1.to_bytes() );
/// let rev_auth2 = ByteArray::<32>::new( rev_pubkey2.to_bytes() );
///
/// // Define a multi-sig spec with 1 of 2
/// let authority_spec = AuthoritySpecInput {
///     sigs_required: 1,
///     authorized_signers: vec![
///         rev_auth1,
///         rev_auth2,
///     ],
/// };
///
/// let result = deepkey_csr::change_rule::update_change_rule(UpdateChangeRuleInput {
///     authority_spec,
///     authorizations: None, // Not required for the first update because the cell agent
///                           // can sign the 'authority_spec' in the coordinator function
/// });
/// # Ok(())
/// # }
/// ```
///
/// Now that the authorized signers are set to keys outside of Lair, the `authorizations` signatures
/// must be provided in the call.
///
/// #### Example usage (second update)
/// ```rust, no_run
/// # use hdk::prelude::*;
/// # use hc_deepkey_sdk::*;
/// # use rand::rngs::OsRng;
/// # use ed25519_dalek::SigningKey;
/// use ed25519_dalek::Signer;
/// # use serde_bytes::ByteArray;
/// # fn main() -> ExternResult<()> {
/// # let rev_key1 = SigningKey::generate(&mut OsRng);
/// # let rev_key2 = SigningKey::generate(&mut OsRng);
/// # let rev_pubkey1 = rev_key1.verifying_key();
/// # let rev_pubkey2 = rev_key2.verifying_key();
/// # let rev_auth1 = ByteArray::<32>::new( rev_pubkey1.to_bytes() );
/// # let rev_auth2 = ByteArray::<32>::new( rev_pubkey2.to_bytes() );
///
/// let rev_key3 = SigningKey::generate(&mut OsRng);
/// let rev_key4 = SigningKey::generate(&mut OsRng);
/// let rev_key5 = SigningKey::generate(&mut OsRng);
///
/// let rev_pubkey3 = rev_key3.verifying_key();
/// let rev_pubkey4 = rev_key4.verifying_key();
/// let rev_pubkey5 = rev_key4.verifying_key();
///
/// let rev_auth3 = ByteArray::<32>::new( rev_pubkey3.to_bytes() );
/// let rev_auth4 = ByteArray::<32>::new( rev_pubkey4.to_bytes() );
/// let rev_auth5 = ByteArray::<32>::new( rev_pubkey4.to_bytes() );
///
/// // Define a multi-sig spec with 2 of 3
/// let authority_spec = AuthoritySpecInput {
///     sigs_required: 2,
///     authorized_signers: vec![
///         rev_auth3,
///         rev_auth4,
///         rev_auth5,
///     ],
/// };
/// // Serialize spec for signing
/// let serialized = deepkey::utils::serialize( &authority_spec )?;
///
/// let result = deepkey_csr::change_rule::update_change_rule(UpdateChangeRuleInput {
///     authority_spec,
///     authorizations: Some(vec![
///         // Sign new spec with the 2nd authorized signer from the previous rule.  The previous
///         // spec only requires 1 signature.
///         ( 1, Signature( rev_key2.sign( &serialized ).to_bytes() ) ),
///     ]),
/// });
/// # Ok(())
/// # }
/// ```
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

    let create_change_rule_record = utils::query_entry_type( EntryTypesUnit::ChangeRule )?
        .first()
        .ok_or(guest_error!(format!(
            "There is no change rule to update"
        )))?
        .to_owned();

    ChangeRule::try_from( create_change_rule_record.clone() )?;

    let keyset_root_addr = utils::query_keyset_root_addr()?;
    let new_change_rule = ChangeRule::new(
        keyset_root_addr.clone(),
        spec_change,
    );

    let update_addr = update_entry(
        create_change_rule_record.action_address().to_owned(),
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
