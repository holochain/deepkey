import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-basic", process.env.LOG_LEVEL );

// import why				from 'why-is-node-running';

// import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import json				from '@whi/json';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import HolochainBackdrop		from '@spartan-hc/holochain-backdrop';
const { Holochain }			= HolochainBackdrop;

import {
    DeepKeyCell,
}					from '@holochain/deepkey-zomelets';
import {
    AppInterfaceClient,
}					from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}					from '../utils.js';
import {
    KeyStore,
}					from '../key_store.js';


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DEEPKEY_DNA_PATH			= path.join( __dirname, "../../dnas/deepkey.dna" );
const DEEPKEY_DNA_NAME			= "deepkey";

const dna1_hash				= new DnaHash( crypto.randomBytes( 32 ) );

const ALICE1_DEVICE_SEED		= Buffer.from("jJQhp80zPT+XBMOZmtfwdBqY9ay9k2w520iwaet1if4=", "base64");
const ALICE2_DEVICE_SEED		= Buffer.from("qSKAyTvyer6o1auniyUiR4JayCcB5qxfwL3PE8oBakc=", "base64");

const alice1_key_store			= new KeyStore( ALICE1_DEVICE_SEED, "alice1" );
const alice2_key_store			= new KeyStore( ALICE2_DEVICE_SEED, "alice2" );

const alice1_app1_id			= "alice1-app1";
const alice2_app1_id			= "alice2-app1";

let APP_PORT;


describe("DeepKey", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	await holochain.install([
	    "alice1",
	    "alice2",
	], {
	    "app_name": "test",
	    "bundle": {
		[DEEPKEY_DNA_NAME]:	DEEPKEY_DNA_PATH,
	    },
	    "membrane_proofs": {
		[DEEPKEY_DNA_NAME]:	{
		    "joining_proof":	crypto.randomBytes( 32 ),
		},
	    },
	});

	APP_PORT			= await holochain.ensureAppPort();
    });

    linearSuite("Key Management", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function basic_tests () {
    let client;
    let alice1_client, alice2_client;
    let deepkey;
    let alice1_deepkey, alice2_deepkey;
    let ksr1_addr;

    let alice1_key1a_reg, alice1_key1a_reg_addr, alice1_key1a;
    let alice1_key1b_reg, alice1_key1b_reg_addr, alice1_key1b;
    let alice1_key1c_reg, alice1_key1c_reg_addr;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	alice1_client			= await client.app( "test-alice1" );
	alice2_client			= await client.app( "test-alice2" );

	{
	    ({
		deepkey,
	    }				= alice1_client.createInterface({
		[DEEPKEY_DNA_NAME]:	DeepKeyCell,
	    }));

	    alice1_deepkey		= deepkey.zomes.deepkey_csr.functions;
	}
	{
	    ({
		deepkey,
	    }				= alice2_client.createInterface({
		[DEEPKEY_DNA_NAME]:	DeepKeyCell,
	    }));

	    alice2_deepkey		= deepkey.zomes.deepkey_csr.functions;
	}

	ksr1_addr			= await alice1_deepkey.query_keyset_authority_action_hash();
    });

    // it("should query (alice1) KSR keyset members (1)", async function () {
    // 	const members			= await alice1_deepkey.query_keyset_members( ksr1_addr );
    // 	log.normal("Keyset Root -> Members: %s", json.debug(members) );

    // 	expect( members			).to.have.length( 1 );
    // });

    // it("should query (alice1) KSR keyset keys with authors (0)", async function () {
    // 	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
    // 	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

    // 	expect( keys			).to.have.length( 0 );
    // });

    it("should register new key (alice1)", async function () {
	this.timeout( 5_000 );

	const derivation_details	= await alice1_deepkey.next_derivation_details( alice1_app1_id );
	const {
	    app_index,
	    key_index,
	}				= derivation_details;
	const path			= `app/${app_index}/key/${key_index}`;
	const new_key			= await alice1_key_store.createKey( path );

	const [ addr, key_reg, key_meta ]	= await alice1_deepkey.create_key({
	    "app_binding": {
		"app_name":		"Alice1 - App #1",
		"installed_app_id":	alice1_app1_id,
		"dna_hashes":		[ dna1_hash ],
	    },
	    "key_generation": {
		"new_key":			await new_key.getAgent(),
		"new_key_signing_of_author":	await new_key.sign( alice1_client.agent_id ),
	    },
	    "derivation_details":	derivation_details,
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (create) addr: %s", addr );

	alice1_key1a			= await new_key.getBytes();
	alice1_key1a_reg		= key_reg;
	alice1_key1a_reg_addr		= addr;

	{
	    const key_state		= await alice1_deepkey.key_state( alice1_key1a );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Valid" );
	}
    });

    it("should register new key (alice2)", async function () {
	this.timeout( 5_000 );

	const derivation_details	= await alice2_deepkey.next_derivation_details( alice2_app1_id );
	const {
	    app_index,
	    key_index,
	}				= derivation_details;
	const path			= `app/${app_index}/key/${key_index}`;
	const new_key			= await alice2_key_store.createKey( path );

	expect( derivation_details	).to.deep.equal({
	    "app_index": 0,
	    "key_index": 0,
	});

	const [ addr, key_reg, key_meta ]	= await alice2_deepkey.create_key({
	    "app_binding": {
		"app_name":		"Alice2 - App #1",
		"installed_app_id":	alice2_app1_id,
		"dna_hashes":		[ dna1_hash ],
	    },
	    "key_generation": {
		"new_key":			await new_key.getAgent(),
		"new_key_signing_of_author":	await new_key.sign( alice2_client.agent_id ),
	    },
	    "derivation_details":	derivation_details,
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (update) addr: %s", addr );

	expect( key_meta.key_index	).to.equal( 0 );
    });

    // it("should query (alice1) KSR keyset keys with authors (1)", async function () {
    // 	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
    // 	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

    // 	expect( keys			).to.have.length( 1 );
    // });

    it("should query (alice1) keyset keys (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_app_keys();
	log.normal("Keyset keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    // it("should query (alice1) KSR keyset keys (1)", async function () {
    // 	const keys			= await alice1_deepkey.query_keyset_keys( ksr1_addr );
    // 	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

    // 	expect( keys			).to.have.length( 1 );
    // });

    // it("should get (alice1) keyset root", async function () {
    // 	const ksr			= await alice1_deepkey.get_keyset_root( ksr1_addr );
    // 	log.normal("Keyset Root: %s", json.debug(ksr) );
    // });

    it("should update key (alice1)", async function () {
	this.timeout( 5_000 );

	const derivation_details	= await alice1_deepkey.next_derivation_details( alice1_app1_id );
	const {
	    app_index,
	    key_index,
	}				= derivation_details;
	const path			= `app/${app_index}/key/${key_index}`;
	const new_key			= await alice1_key_store.createKey( path );

	expect( derivation_details	).to.deep.equal({
	    "app_index": 0,
	    "key_index": 1,
	});

	const [ addr, key_reg, key_meta ]	= await alice1_deepkey.update_key({
	    "installed_app_id":		alice1_app1_id,
	    "key_revocation": {
		"prior_key_registration": alice1_key1a_reg_addr,
		"revocation_authorization": [
		    [ 0, crypto.randomBytes(64) ],
		],
	    },
	    "key_generation": {
		"new_key":			await new_key.getAgent(),
		"new_key_signing_of_author":	await new_key.sign( alice1_client.agent_id ),
	    },
	    "derivation_details":	derivation_details,
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (update) addr: %s", addr );

	alice1_key1b			= await new_key.getBytes();
	alice1_key1b_reg		= key_reg;
	alice1_key1b_reg_addr		= addr;

	expect( key_meta.key_index	).to.equal( 1 );

	{
	    const key_state		= await alice1_deepkey.key_state( alice1_key1a );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Invalidated" );
	}
	{
	    const key_state		= await alice1_deepkey.key_state( alice1_key1b );
	    log.normal("Key (1b) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Valid" );
	}
    });

    it("should query (alice1) keyset keys (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_app_keys();
	log.normal("Keyset keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    let invite_accept;
    it("(alice1) should invite device 'alice2'", async function () {
	// const parent			= await alice1_deepkey.query_keyset_authority_action_hash();
	// {
	//     "keyset_root":	ksr1_addr,
	//     "parent":		parent, // Either the KeysetRoot or the DeviceInviteAcceptance
	//     "invitee":		alice2_client.cell_client,
	// });
	invite_accept			= await alice1_deepkey.invite_agent( alice2_client.agent_id );
	log.normal("Device Invite Acceptance: %s", json.debug(invite_accept) );
    });

    it("(alice2) should accept invite from 'alice1'", async function () {
	const acceptance_addr		= await alice2_deepkey.accept_invite( invite_accept );
	log.normal("Acceptance [addr]: %s", acceptance_addr );
    });

    it("should query (alice1) keyset members (2)", async function () {
	const members			= await alice1_deepkey.query_keyset_members( ksr1_addr );
	log.normal("Keyset Root -> Members: %s", json.debug(members) );

	expect( members			).to.have.length( 2 );
    });

    it("should get (alice1) KSR members (2)", async function () {
	const members			= await alice1_deepkey.get_ksr_members( ksr1_addr );
	log.normal("Members (devices): %s", json.debug(members) );

	expect( members			).to.have.length( 2 );
	expect( members[0]		).to.deep.equal( alice1_client.agent_id );
	expect( members[1]		).to.deep.equal( alice2_client.agent_id );
    });

    // it("should get (alice1) KSR keys (1)", async function () {
    // 	const devices			= await alice1_deepkey.get_keysets_for_ksr( ksr1_addr );
    // 	log.normal("KSR keyset: %s", json.debug(devices) );

    // 	expect( devices			).to.have.length( 2 );
    // });

    // it("should query (alice1) keyset keys (1)", async function () {
    // 	this.skip();

    // 	const keys			= await alice1_deepkey.query_keyset_keys( ksr1_addr );
    // 	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

    // 	expect( keys			).to.have.length( 1 );
    // });

    // it("should query (alice1) keyset keys with authors (1)", async function () {
    // 	this.skip();

    // 	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
    // 	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

    // 	expect( keys			).to.have.length( 1 );
    // });

    // it("should query (alice1) app bindings", async function () {
    // 	this.skip();

    // 	let app_bindings			= await alice1_deepkey.query_app_bindings();
    // 	log.normal("App Bindings: %s", json.debug(app_bindings) );

    // 	expect( app_bindings		).to.have.length( 1 );
    // });

    it("should revoke key (alice1)", async function () {
	this.timeout( 5_000 );

	const [ addr, key_reg ]	= await alice1_deepkey.revoke_key({
	    "installed_app_id":		alice1_app1_id,
	    "key_revocation": {
		"prior_key_registration": alice1_key1b_reg_addr,
		"revocation_authorization": [
		    [ 0, crypto.randomBytes(64) ],
		],
	    },
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key registration (update) addr: %s", addr );

	alice1_key1c_reg		= key_reg;
	alice1_key1c_reg_addr		= addr;

	{
	    const key_state		= await alice1_deepkey.key_state( alice1_key1a );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Invalidated" );
	}
	{
	    const key_state		= await alice1_deepkey.key_state( alice1_key1b );
	    log.normal("Key (1b) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Invalidated" );
	}
    });

    it("should check key state before creation (alice1)", async function () {
	const timestamp			= Date.now() - (60 * 60 * 1000); // 1 hour ago
	const key_state			= await alice1_deepkey.key_state([ alice1_key1a, timestamp ]);
	log.normal("Key (1a) state @ %s: %s", timestamp, json.debug(key_state) );

	expect( key_state		).to.have.key( "NotFound" );
    });

    linearSuite("Errors", function () {

	it("should fail to register invalid key", async function () {
	    await expect_reject(async () => {
		await alice1_deepkey.create_key({
		    "app_binding": {
			"app_name":		"?",
			"installed_app_id":	"?",
			"dna_hashes":		[ dna1_hash ],
		    },
		    "key_generation": {
			"new_key":			new AgentPubKey( crypto.randomBytes( 32 ) ),
			"new_key_signing_of_author":	crypto.randomBytes( 64 ),
		    },
		    "derivation_details": {
			"app_index": 1,
			"key_index": 0,
		    },
		});
	    }, "Signature does not match new key" );
	});

    });

    after(async function () {
	await client.close();
    });
}
