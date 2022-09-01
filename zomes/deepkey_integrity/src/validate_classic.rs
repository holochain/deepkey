use hdk::prelude::*;

/// 
/// The validation package for classic (hdk v0.0.128 and prior) style validation functions
/// 
/// The Element is now called a Record; it contains a SignedActionHashed (what used to be called a
/// "Header", and a RecordEntry, previously an "Entry").  Convert to a ValidateData containing the
/// element being validated, and optionally a ValidationPackage containing a sequence of Vec<Record>
/// representing the source chain Record entries and the Action headers.
/// 
/// The new Vec<RegisterAgentActivity> (RegisterAgentAcivity.action is a SignedActionHashed =
/// SignedHashed<Action>) includes agent's Action (but *not* the RecordEntry), so this interface
/// doesn't include all information useful for validation; we must get the RecordEntry to match
/// the Record{ signed_action, entry } included in the old ValidationPackage(Vec<Record>).
/// 
/// It is the responsibility of the caller to ensure that the correct amount of the source-chain is
/// provided, as required by the classic ValidateData configuration.
/// 
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, SerializedBytes)]
pub struct ValidateData {
    pub element: Record, // was known as Element
    pub validation_package: Option<ValidationPackage>, // a (Vec<Record>)
}

impl ValidateData {
    pub fn new(element: Record, validation_package: Option<ValidationPackage>) -> Self {
        Self {
            element,
            validation_package,
        }
    }

    pub fn new_element_only(element: Record) -> Self {
        Self {
            element,
            validation_package: None,
        }
    }
}

/// Convert a Record (eg. the SignedHashed<Action> and RecordEntry being committed) and a chain
/// consisting of Vec<RegisterAgentActivity> (eg. from must_get_agent_activity) into a classic
/// ValidateData package.
/// 
/// This involves using must_get_entry to retrieve any public RecordEntry::Present(Entry) data.  For each
/// Action that involves a public Entry, retrieve it and construct the ValidationPackage(Vec<Record>)
/// 
impl TryFrom<(Record,Vec<RegisterAgentActivity>)> for ValidateData {
    type Error = WasmError;
    fn try_from(rc: (Record,Vec<RegisterAgentActivity>)) -> Result<Self, Self::Error> {
	let element: Record = rc.0;
	let records: Vec<Record> = rc.1.into_iter()
	    .map(|signed_action| { // a RegisterAgentActivity
                Ok(match signed_action.action.hashed.content { // a SignedHashed<Action>'s &Action
                    Action::Dna(_) | Action::AgentValidationPkg(_) | Action::InitZomesComplete(_) |
                    Action::CreateLink(_) | Action::DeleteLink(_) |
                    Action::OpenChain(_) | Action::CloseChain(_) |
                    Action::Delete(_) =>
                        Record { signed_action: signed_action.action, entry: RecordEntry::NotApplicable },
                    Action::Create(Create { ref entry_type, ref entry_hash, .. }) |
                    Action::Update(Update { ref entry_type, ref entry_hash, .. }) => {
		        match entry_type.visibility() {
			    EntryVisibility::Public => Record {
			        signed_action: signed_action.action.to_owned(), entry: RecordEntry::Present(must_get_entry( entry_hash.to_owned() )?.content) },
    			    _ => Record { signed_action: signed_action.action.to_owned(), entry: RecordEntry::Hidden },
                        }
                    },
                })
            })
	    .collect::<ExternResult<_>>()?;
	Ok(ValidateData { element, validation_package: Some(ValidationPackage(records)) })
    }
}
