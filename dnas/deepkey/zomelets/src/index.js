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
    DnaBinding,
    DeviceInvite,
    DeviceInviteAcceptance,

    KeyRegistrationEntry,
}					from './types.js';


export const DeepKeyCSRZomelet		= new Zomelet({
    async query_local_key_info () {
	const result			= await this.call();

	return result;
    },
    async query_keyset_authority_action_hash () {
	const result			= await this.call();

	return new ActionHash( result );
    },
    async query_keyset_root_action_hash () {
	const result			= await this.call();

	return new ActionHash( result );
    },
    async get_keyset_root ( input ) {
	const result			= await this.call( input );

	return KeysetRoot( result );
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
    async register_key ({ key, signature, dna_hash, app_name }) {
	const result			= await this.call([
	    key, signature, dna_hash, app_name
	]);

	return new ActionHash( result );
    },
    async invite_agent ( input ) {
	const result			= await this.call( input );

	return DeviceInviteAcceptance( result );
    },


    //
    // Virtual functions
    //
    // async save_integrity ( bytes ) {
    // 	const addr			= await this.zomes.mere_memory_api.save( bytes );

    // 	return await this.functions.create_wasm({
    // 	    "wasm_type": WASM_TYPES.INTEGRITY,
    // 	    "mere_memory_addr": addr,
    // 	});
    // },
});


export const DeepKeyCell		= new CellZomelets({
    "deepkey": DeepKeyCSRZomelet,
});


export *				from './types.js';

export default {
    // Zomelets
    DeepKeyCSRZomelet,

    // CellZomelets
    DeepKeyCell,
};
