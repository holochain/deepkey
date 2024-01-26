import { Logger }			from '@whi/weblogger';
const log				= new Logger("test-basic", process.env.LOG_LEVEL );

// import why				from 'why-is-node-running';

// import * as fs				from 'node:fs/promises';
import path				from 'path';
import crypto				from 'crypto';

import { expect }			from 'chai';

import * as ed				from '@noble/ed25519';
import { hmac }				from '@noble/hashes/hmac';

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


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DEEPKEY_DNA_PATH			= path.join( __dirname, "../../dnas/deepkey.dna" );
const DEEPKEY_DNA_NAME			= "deepkey";

const dna1_hash				= new DnaHash( crypto.randomBytes( 32 ) );

const revocation_key1			= ed.utils.randomPrivateKey();
const revocation_key2			= ed.utils.randomPrivateKey();

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

    it("should query (alice1) KSR keyset members (1)", async function () {
	const members			= await alice1_deepkey.query_keyset_members( ksr1_addr );
	log.normal("Keyset Root -> Members: %s", json.debug(members) );

	expect( members			).to.have.length( 1 );
    });

    it("should query (alice1) KSR keyset keys with authors (0)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 0 );
    });

    it("should register new key (alice1)", async function () {
	this.timeout( 5_000 );

	const secret			= ed.utils.randomPrivateKey();
	const pubkey_bytes		= await ed.getPublicKeyAsync( secret );

	const registration_addr		= await alice1_deepkey.register_key({
	    "app_name":		"Alice1 - App #1",
	    "dna_hashes":	[ dna1_hash ],
	    "key":		new AgentPubKey( pubkey_bytes ),
	    "signature":	await ed.signAsync( alice1_client.agent_id, secret ),
	});
	log.normal("Key registration addr: %s", registration_addr );
    });

    it("should register new key (alice2)", async function () {
	this.timeout( 5_000 );

	const secret			= ed.utils.randomPrivateKey();
	const pubkey_bytes		= await ed.getPublicKeyAsync( secret );

	const registration_addr		= await alice2_deepkey.register_key({
	    "app_name":		"Alice2 - App #1",
	    "dna_hashes":	[ dna1_hash ],
	    "key":		new AgentPubKey( pubkey_bytes ),
	    "signature":	await ed.signAsync( alice2_client.agent_id, secret ),
	});
	log.normal("Key registration addr: %s", registration_addr );
    });

    it("should query (alice1) KSR keyset keys with authors (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query (alice1) KSR keyset keys (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query (alice1) KSR keyset keys (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should get (alice1) keyset root", async function () {
	const ksr			= await alice1_deepkey.get_keyset_root( ksr1_addr );
	log.normal("Keyset Root: %s", json.debug(ksr) );
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

    it("should get (alice1) KSR keys (1)", async function () {
	const devices			= await alice1_deepkey.get_keysets_for_ksr( ksr1_addr );
	log.normal("KSR keysets: %s", json.debug(devices) );

	expect( devices			).to.have.length( 2 );
    });

    it("should query (alice1) keyset keys (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query (alice1) keyset keys with authors (1)", async function () {
	const keys			= await alice1_deepkey.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query (alice1) local key info", async function () {
	let key_info			= await alice1_deepkey.query_local_key_info();
	log.normal("Key info: %s", json.debug(key_info) );

	expect( key_info		).to.have.length( 1 );
    });

    linearSuite("Errors", function () {

	it("should fail to register invalid key", async function () {
	    await expect_reject(async () => {
		await alice1_deepkey.register_key({
		    "app_name":		"?",
		    "dna_hashes":	[ dna1_hash ],
		    "key":		new AgentPubKey( crypto.randomBytes( 32 ) ),
		    "signature":	crypto.randomBytes( 64 ),
		});
	    }, "Signature does not match new key" );
	});

    });

    after(async function () {
	await client.close();
    });
}
