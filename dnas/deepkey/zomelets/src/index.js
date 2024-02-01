import {
    AnyDhtHash,
    AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash'; // approx. 11kb
import {
    Zomelet,
    CellZomelets,
}					from '@spartan-hc/zomelets'; // approx. 7kb
import {
    Signature,
    SignedAction,
    Authorization,
    AuthoritySpec,
    AuthorizedSpecChange,
    ChangeRule,
    KeysetRoot,
    KeyMeta,
    KeyAnchor,
    KeyState,
    AppBinding,
    DeviceInvite,
    DeviceInviteAcceptance,

    KeyRegistrationEntry,
    KeyInfo,
}					from './types.js';


const functions				= {
    // Local reading
    async query_key_info () {
	const result			= await this.call();

	return result.map( info => KeyInfo( info ) );
    },
    async query_keyset_authority_action_hash () {
	const result			= await this.call();

	return new ActionHash( result );
    },
    async query_keyset_root_action_hash () {
	const result			= await this.call();

	return new ActionHash( result );
    },
    async query_keyset_members ( input ) {
	const result			= await this.call( input );

	return result.map( pubkey => new AgentPubKey(pubkey) );
    },
    async query_keyset_keys_with_authors ( input ) {
	const result			= await this.call( input );

	return result.map( ([author, key_registration]) => [
	    new AgentPubKey(author), KeyRegistrationEntry( key_registration )
	]);
    },
    async query_keyset_keys ( input ) {
	const result			= await this.call( input );

	return result.map( key_registration => KeyRegistrationEntry( key_registration ) );
    },
    async query_app_bindings ( input ) {
	const result			= await this.call( input );

	return result.map( entry => AppBinding( entry ) );
    },

    // Public reading
    async get_keyset_root ( input ) {
	const result			= await this.call( input );

	return KeysetRoot( result );
    },
    async get_ksr_members ( input ) {
	const result			= await this.call( input );

	return result.map( agent => new AgentPubKey(agent) );
    },
    async get_device_keys ( input ) {
	const result			= await this.call( input );

	return result.map( key_anchor => KeyAnchor( key_anchor ) );
    },
    async key_state ( input, options ) {
	if ( !Array.isArray( input ) )
	    input			= [ input, Date.now() ];

	// Because the 'Timestamp' type on the other side expects nano seconds
	if ( options?.adjust_for_nano_seconds !== false )
	    input[1]		       *= 1000;

	const result			= await this.call( input );

	return KeyState( result );
    },

    // Key Registration
    async next_derivation_details ( input ) {
	const result			= await this.call( input );

	return result;
    },
    async create_key ( input ) {
	const result			= await this.call( input );

	return [
	    new ActionHash( result[0] ),
	    KeyRegistrationEntry( result[1] ),
	    KeyMeta( result[2] ),
	];
    },
    async update_key ( input ) {
	const result			= await this.call( input );

	return [
	    new ActionHash( result[0] ),
	    KeyRegistrationEntry( result[1] ),
	    KeyMeta( result[2] ),
	];
    },
    async revoke_key ( input ) {
	const result			= await this.call( input );

	return [
	    new ActionHash( result[0] ),
	    KeyRegistrationEntry( result[1] ),
	];
    },

    // Device Inviting
    async invite_agent ( input ) {
	const result			= await this.call( input );

	return DeviceInviteAcceptance( result );
    },
    async accept_invite ( input ) {
	const result			= await this.call( input );

	return new ActionHash( result );
    },

    // Change Rules
    async update_change_rule ( input ) {
	const result			= await this.call( input );

	return new ChangeRule( result );
    },
    async construct_authority_spec ( input ) {
	const result			= await this.call( input );

	return {
	    "authority_spec": AuthoritySpec( result[0] ),
	    "serialized": new Uint8Array( result[1] ),
	};
    },
    async get_current_change_rule_for_ksr ( input ) {
	const result			= await this.call( input );

	return new ChangeRule( result );
    },
    "get_ksr_change_rule_links": true,


    //
    // Virtual functions
    //
    async get_keysets_for_ksr ( input ) {
	const devices			= [];
	const members			= await this.functions.get_ksr_members( input );

	for ( let agent of members ) {
	    devices.push({
		"member": agent,
		"keys": await this.functions.get_device_keys( agent ),
	    });
	}

	return devices;
    },
};

const APP_ENTRY_STRUCTS_MAP		= {
    ChangeRule,
    KeysetRoot,
    KeyMeta,
    KeyAnchor,
    AppBinding,
    DeviceInvite,
    DeviceInviteAcceptance,
    "KeyRegistration": KeyRegistrationEntry,
};

function formatSignal ( signal ) {
    if ( signal.action ) {
	signal.signed_action		= SignedAction( signal.action );
	signal.action			= signal.signed_action.hashed.content;
    }

    if ( signal.app_entry ) {
	const app_entry_type		= signal.app_entry.type;
	const struct			= APP_ENTRY_STRUCTS_MAP[ app_entry_type ];

	if ( struct === undefined )
	    throw new TypeError(`No AppEntry struct for type '${app_entry_type}'`);

	signal.app_entry_type		= app_entry_type;
	signal.app_entry		= struct( signal.app_entry );
    }

    // console.log("Signal", JSON.stringify(signal,null,4) );
    return signal;
}

const signals				= {
    EntryCreated ( signal ) {
	formatSignal( signal );

	// if ( signal.action ) {
	//     console.log(
	// 	"  %s Action => [%s]",
	// 	signal.action.type, signal.signed_action.hashed.hash, JSON.stringify(signal.action,null,4)
	//     );
	// }

	// if ( signal.app_entry ) {
	//     console.log(
	// 	"SIGNAL: AppEntry => [%s]", signal.app_entry_type, JSON.stringify(signal.app_entry,null,4)
	//     );
	// }
    },
    LinkCreated ( signal ) {
	formatSignal( signal );

	// console.log(
	//     "SIGNAL: LinkType => [%s]", signal.action.type, signal.link_type
	// );
    },
};

export const DeepKeyCSRZomelet		= new Zomelet({
    functions,
    signals,
});


export const DeepKeyCell		= new CellZomelets({
    "deepkey_csr": DeepKeyCSRZomelet,
});


export *				from './types.js';

export default {
    // Zomelets
    DeepKeyCSRZomelet,

    // CellZomelets
    DeepKeyCell,
};
