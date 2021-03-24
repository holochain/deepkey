use hdk::prelude::*;
use crate::error::Error;

pub struct ResolvedDependency<D>(pub Element, pub D);

pub fn resolve_dependency<'a, O>(hash: AnyDhtHash) -> ExternResult<Result<ResolvedDependency<O>, ValidateCallbackResult>>
    where
        O: TryFrom<SerializedBytes, Error = SerializedBytesError>        {
    let element = match get(hash.clone(), GetOptions::content())? {
        Some(element) => element,
        None => return Ok(Err(ValidateCallbackResult::UnresolvedDependencies(vec![hash]))),
    };

    let output: O = match element.entry().to_app_option() {
        Ok(Some(output)) => output,
        Ok(None) => return Ok(Err(Error::EntryMissing.into())),
        Err(e) => return Ok(Err(ValidateCallbackResult::Invalid(e.to_string()))),
    };

    Ok(Ok(ResolvedDependency(element, output)))
}