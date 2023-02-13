use hdi::prelude::*;
#[hdk_entry_helper]
#[derive(Clone)]
pub struct AuthorizedSpecChange {
    pub new_spec: ActionHash,
    pub authorization_of_new_spec: Vec<u32>,
}
pub fn validate_create_authorized_spec_change(
    _action: EntryCreationAction,
    _authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(ValidateCallbackResult::Valid)
}
pub fn validate_update_authorized_spec_change(
    _action: Update,
    _authorized_spec_change: AuthorizedSpecChange,
    _original_action: EntryCreationAction,
    _original_authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Authorized Spec Changes cannot be updated"),
        ),
    )
}
pub fn validate_delete_authorized_spec_change(
    _action: Delete,
    _original_action: EntryCreationAction,
    _original_authorized_spec_change: AuthorizedSpecChange,
) -> ExternResult<ValidateCallbackResult> {
    Ok(
        ValidateCallbackResult::Invalid(
            String::from("Authorized Spec Changes cannot be deleted"),
        ),
    )
}
