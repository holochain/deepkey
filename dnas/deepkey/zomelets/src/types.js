
import { Bytes }			from '@whi/bytes-class';
import {
    AgentPubKey, DnaHash,
    ActionHash, EntryHash,
    AnyLinkableHash,
}					from '@spartan-hc/holo-hash';
import {
    // ScopedEntity,
    intoStruct,
    AnyType, OptionType, None,
    VecType, MapType,
}					from '@whi/into-struct';


export const Signature			= Bytes;


export class EntryTypeEnum {
    constructor ( data ) {
	if ( "App" in data )
	    return intoStruct( data, AppEntryTypeStruct );

	console.log("EntryTypeEnum constructor:", data );
	throw new Error(`Unhandled Action entry type: ${Object.keys(data)[0]}`);
    }
}

export const AppEntryTypeStruct		= {
    "App": {
        "entry_index":		Number,
        "zome_index":		Number,
        "visibility":		AnyType,
    },
};

export const WeightStruct		= {
    "bucket_id":		Number,
    "units":			Number,
    "rate_bytes":		OptionType( Number ),
};

export const ActionBaseStruct		= {
    "type": 			String,
    "author": 			AgentPubKey,
    "timestamp":		Number,
    "action_seq":		Number,
    "prev_action":		OptionType( ActionHash ),
}
export const CreateActionStruct		= {
    ...ActionBaseStruct,
    "entry_type":		EntryTypeEnum,
    "entry_hash":		EntryHash,
    "weight":			WeightStruct,
};

export const CreateLinkActionStruct	= {
    ...ActionBaseStruct,
    "base_address":		AnyLinkableHash,
    "target_address":		AnyLinkableHash,
    "zome_index":		Number,
    "link_type":		Number,
    "tag":			Bytes,
    "weight":			WeightStruct,
};

export class ActionEnum {
    constructor ( data ) {
	if ( data.type === "Create" )
	    return intoStruct( data, CreateActionStruct );
	if ( data.type === "CreateLink" )
	    return intoStruct( data, CreateLinkActionStruct );

	console.log("ActionEnum constructor:", data );
	throw new Error(`Unhandled Action type: ${data.type}`);
    }
}


export const SignedActionStruct		= {
    "hashed": {
	"content":		ActionEnum,
	"hash":			ActionHash,
    },
    "signature":		Signature,
};

export function SignedAction ( data ) {
    return intoStruct( data, SignedActionStruct );
}


export const AuthorizationStruct	= [ Number, Signature ];

export function Authorization ( data ) {
    return intoStruct( data, AuthorizationStruct );
}


export const AuthoritySpecStruct	= {
    "sigs_required":		Number,
    "authorized_signers":	VecType( Bytes ),
};

export function AuthoritySpec ( data ) {
    return intoStruct( data, AuthoritySpecStruct );
}


export const AuthorizedSpecChangeStruct	= {
    "new_spec":				AuthoritySpecStruct,
    "authorization_of_new_spec":	VecType( AuthorizationStruct ),
};

export function AuthorizedSpecChange ( data ) {
    return intoStruct( data, AuthorizedSpecChangeStruct );
}


export const ChangeRuleStruct		= {
    "keyset_root":		ActionHash,
    "keyset_leaf":		ActionHash,
    "spec_change":		AuthorizedSpecChangeStruct,
};

export function ChangeRule ( data ) {
    return intoStruct( data, ChangeRuleStruct );
}


export const KeysetRootStruct		= {
    "first_deepkey_agent":		AgentPubKey,
    "root_pub_key":			Bytes,
    "signed_fda":			Signature,
};

export function KeysetRoot ( data ) {
    return intoStruct( data, KeysetRootStruct );
}


export const DerivationDetails		= {
    "app_index":		Number,
    "agent_index":		Number,
};

export const KeyMetaStruct		= {
    "key_anchor_addr":			ActionHash,
    "derivation_details":		DerivationDetails,
    // "derivation_path":		Bytes,
    // "key_type":			AnyType,
};

export function KeyMeta ( data ) {
    return intoStruct( data, KeyMetaStruct );
}


export const KeyAnchorStruct		= {
    "bytes":			Bytes,
};

export function KeyAnchor ( data ) {
    return intoStruct( data, KeyAnchorStruct );
}


export const AppBindingStruct		= {
    "key_meta_addr":		ActionHash,
    "dna_hashes":		VecType( DnaHash ),
    "app_name":			String,
};

export function AppBinding ( data ) {
    return intoStruct( data, AppBindingStruct );
}


export const DeviceInviteStruct		= {
    "keyset_root":		ActionHash,
    "parent":			ActionHash,
    "invitee":			AgentPubKey,
};

export function DeviceInvite ( data ) {
    return intoStruct( data, DeviceInviteStruct );
}


export const DeviceInviteAcceptanceStruct = {
    "keyset_root_authority":	ActionHash,
    "invite":			ActionHash,
};

export function DeviceInviteAcceptance ( data ) {
    return intoStruct( data, DeviceInviteAcceptanceStruct );
}


export const KeyGenerationStruct = {
    "new_key":				AgentPubKey,
    "new_key_signing_of_author":	Signature,
};
export const KeyRevocationStruct = {
    "prior_key_registration":		ActionHash,
    "revocation_authorization":		VecType( AuthorizationStruct ),
};

export function KeyRegistrationEntry ( entry ) {
    if ( "Create" in entry )
	entry.Create		= intoStruct( entry.Create, KeyGenerationStruct );
    else if ( "CreateOnly" in entry )
	entry.CreateOnly	= intoStruct( entry.CreateOnly, KeyGenerationStruct );
    else if ( "Update" in entry )
	entry.Update		= intoStruct( entry.Update, [ KeyRevocationStruct, KeyGenerationStruct ] );
    else if ( "Delete" in entry )
	entry.Delete		= intoStruct( entry.Delete, KeyRevocationStruct );
    else
	throw new TypeError(`Unknown type for KeyRegistration entry: ${Object.keys(entry)[0]}`);

    return entry;
}

// export class KeyRegistration extends ScopedEntity {
//     static STRUCT		= KeyRegistrationStruct;
// }

export const KeyInfoStruct = [
    AppBindingStruct,
    KeyMetaStruct,
    KeyRegistrationEntry,
];

export function KeyInfo ( data ) {
    return intoStruct( data, KeyInfoStruct );
}


export default {
    Signature,

    SignedActionStruct,
    SignedAction,

    AuthorizationStruct,
    Authorization,

    AuthoritySpecStruct,
    AuthoritySpec,

    AuthorizedSpecChangeStruct,
    AuthorizedSpecChange,

    ChangeRuleStruct,
    ChangeRule,

    KeysetRootStruct,
    KeysetRoot,

    KeyMetaStruct,
    KeyMeta,

    KeyAnchorStruct,
    KeyAnchor,

    AppBindingStruct,
    AppBinding,

    DeviceInviteStruct,
    DeviceInvite,

    DeviceInviteAcceptanceStruct,
    DeviceInviteAcceptance,

    KeyGenerationStruct,
    KeyRevocationStruct,
    KeyRegistrationEntry,

    KeyInfoStruct,
    KeyInfo,
};
