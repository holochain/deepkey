import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-basic", process.env.LOG_LEVEL );

// import why				from 'why-is-node-running';

import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import * as ed				from '@noble/ed25519';

import json				from '@whi/json';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}					from '@spartan-hc/holo-hash';
import { Holochain }			from '@spartan-hc/holochain-backdrop';

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

const dna1_hash				= new DnaHash( crypto.randomBytes( 32 ) );

const alice1_app1_id			= "alice1-app1";
const alice2_app1_id			= "alice2-app1";

const ALICE1_DEVICE_SEED		= Buffer.from("jJQhp80zPT+XBMOZmtfwdBqY9ay9k2w520iwaet1if4=", "base64");
const alice1_key_store			= new KeyStore( ALICE1_DEVICE_SEED, "alice1" );

const revocation_key1			= ed.utils.randomPrivateKey();
const revocation_key2			= ed.utils.randomPrivateKey();
const rev1_pubkey			= await ed.getPublicKeyAsync( revocation_key1 );
const rev2_pubkey			= await ed.getPublicKeyAsync( revocation_key2 );

let app_port;
let installations;


describe("DeepKey", function () {
    const holochain			= new Holochain({
	"timeout": 20_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	installations			= await holochain.install([
	    "alice_host",
	    "alice1",
	    "alice2",
	], {
	    "app_name": "test",
	    "bundle": {
		"deepkey":	DEEPKEY_DNA_PATH,
	    },
	});

	app_port			= await holochain.ensureAppPort();
    });

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function basic_tests () {
    let client;

    // Hosted Alice deepkey
    let hosted_alice_client;
    let hosted_alice_deepkey;

    // Alice deepkey
    let alice1_client;
    let alice1_deepkey;
    let alice2_client;
    let alice2_deepkey;

    // Alice key1
    const app_index			= 1; // 0 is reserved for the deepkey cell agent
    const key_index			= 0;
    const key1a_path			= `app/${app_index}/key/${key_index}`;
    let alice1_key1a;

    // Hosted Alice key1
    let hosted_alice_key1a_reg, hosted_alice_key1a_reg_addr;
    // Alice1 key1
    let alice1_key1a_reg, alice1_key1a_reg_addr;
    // Alice2 key1
    let alice2_key1a_reg, alice2_key1a_reg_addr;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( app_port, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});

	const hosted_alice_token	= installations.alice_host.test.auth.token;
	hosted_alice_client		= await client.app( hosted_alice_token );

	const alice1_token		= installations.alice1.test.auth.token;
	alice1_client			= await client.app( alice1_token );

	const alice2_token		= installations.alice2.test.auth.token;
	alice2_client			= await client.app( alice2_token );

	{
	    const {
		deepkey,
	    }				= hosted_alice_client.createInterface({
		"deepkey":	DeepKeyCell,
	    });

	    hosted_alice_deepkey	= deepkey.zomes.deepkey_csr.functions;
	}
	{
	    const {
		deepkey,
	    }				= alice1_client.createInterface({
		"deepkey":	DeepKeyCell,
	    });

	    alice1_deepkey		= deepkey.zomes.deepkey_csr.functions;
	}
	{
	    const {
		deepkey,
	    }				= alice2_client.createInterface({
		"deepkey":	DeepKeyCell,
	    });

	    alice2_deepkey		= deepkey.zomes.deepkey_csr.functions;
	}

	const hosted_alice_ksr_addr	= await hosted_alice_deepkey.query_keyset_authority_action_hash();
	const alice1_ksr_addr		= await alice1_deepkey.query_keyset_authority_action_hash();
	const alice2_ksr_addr		= await alice2_deepkey.query_keyset_authority_action_hash();

	const auth_spec_package		= await alice1_deepkey.construct_authority_spec({
	    "sigs_required": 1,
	    "authorized_signers": [
		rev1_pubkey,
		rev2_pubkey,
	    ],
	});
	const new_change_rule		= await alice1_deepkey.update_change_rule({
	    "authority_spec": auth_spec_package.authority_spec,
	});
	log.normal("New Change Rule: %s", json.debug(new_change_rule) );

	alice1_key1a			= await alice1_key_store.createKey( key1a_path );
    });

    it("should register unmanaged key (alice)", async function () {
	this.timeout( 5_000 );

	const [ addr, key_reg, key_meta ]	= await hosted_alice_deepkey.create_key({
	    "app_binding": {
		"app_name":		"Alice - App #1",
		"installed_app_id":	alice1_app1_id,
		"dna_hashes":		[ dna1_hash ],
	    },
	    "key_generation": {
		"new_key":			await alice1_key1a.getAgent(),
		"new_key_signing_of_author":	await alice1_key1a.sign( hosted_alice_client.agent_id ),
	    },
	    "derivation_details": {
		app_index,
		key_index,
		"derivation_seed":	alice1_key_store.seed,
		"derivation_bytes":	alice1_key1a.derivation_bytes,
	    },
	    "create_only": true,
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (create) addr: %s", addr );

	hosted_alice_key1a_reg		= key_reg;
	hosted_alice_key1a_reg_addr	= addr;

	{
	    const key_state		= await hosted_alice_deepkey.key_state(
		await alice1_key1a.getBytes()
	    );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Valid" );
	}
    });

    it("should claim unmanaged key", async function () {
	this.timeout( 5_000 );

	const [ addr, key_reg, key_meta ]	= await alice1_deepkey.create_key({
	    "app_binding": {
		"app_name":		"Alice1 - App #1",
		"installed_app_id":	alice1_app1_id,
		"dna_hashes":		[ dna1_hash ],
	    },
	    "key_generation": {
		"new_key":			await alice1_key1a.getAgent(),
		"new_key_signing_of_author":	await alice1_key1a.sign( alice1_client.agent_id ),
	    },
	    "derivation_details":	{
		app_index,
		key_index,
		"derivation_seed":	alice1_key_store.seed,
		"derivation_bytes":	alice1_key1a.derivation_bytes,
	    },
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (update) addr: %s", addr );

	alice1_key1a_reg		= key_reg;
	alice1_key1a_reg_addr		= addr;
    });

    it("should fail to update hosted key", async function () {
	this.timeout( 10_000 );

	await expect_reject(async () => {
	    await hosted_alice_deepkey.revoke_key({
		"key_revocation": {
		    "prior_key_registration": hosted_alice_key1a_reg_addr,
		    "revocation_authorization": [
			[ 0, await alice1_key1a.sign( hosted_alice_key1a_reg_addr ) ],
		    ],
		},
	    });
	}, "cannot be updated" );
    });

    it("should update key", async function () {
	this.timeout( 10_000 );

	const [ addr, key_reg ]	= await alice1_deepkey.revoke_key({
	    "key_revocation": {
		"prior_key_registration": alice1_key1a_reg_addr,
		"revocation_authorization": [
		    [ 0, await ed.signAsync( alice1_key1a_reg_addr, revocation_key1 ) ],
		],
	    },
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key registration (update) addr: %s", addr );

	{
	    const key_state		= await hosted_alice_deepkey.key_state(
		await alice1_key1a.getBytes()
	    );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Invalid" );
	}
    });

    it("should claim unmanaged key with another chain", async function () {
	this.timeout( 5_000 );

	const [ addr, key_reg, key_meta ]	= await alice2_deepkey.create_key({
	    "app_binding": {
		"app_name":		"Alice2 - App #1",
		"installed_app_id":	alice2_app1_id,
		"dna_hashes":		[ dna1_hash ],
	    },
	    "key_generation": {
		"new_key":			await alice1_key1a.getAgent(),
		"new_key_signing_of_author":	await alice1_key1a.sign( alice2_client.agent_id ),
	    },
	    "derivation_details":	{
		app_index,
		key_index,
		"derivation_seed":	alice1_key_store.seed,
		"derivation_bytes":	alice1_key1a.derivation_bytes,
	    },
	});
	log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
	log.normal("Key Meta: %s", json.debug(key_meta) );
	log.normal("Key registration (update) addr: %s", addr );

	alice2_key1a_reg		= key_reg;
	alice2_key1a_reg_addr		= addr;

	{
	    const key_state		= await hosted_alice_deepkey.key_state(
		await alice1_key1a.getBytes()
	    );
	    log.normal("Key (1a) state: %s", json.debug(key_state) );

	    expect( key_state		).to.have.key( "Invalid" );
	}
    });

    after(async function () {
	await client.close();
    });
}
