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


const __dirname				= path.dirname( new URL(import.meta.url).pathname );
const DEEPKEY_DNA_PATH			= path.join( __dirname, "../../dnas/deepkey.dna" );
const APP_PORT				= 23_567;

const DEEPKEY_DNA_NAME			= "deepkey";


describe("DeepKey", function () {
    const holochain			= new Holochain({
	"timeout": 60_000,
	"default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
	this.timeout( 60_000 );

	await holochain.backdrop({
	    "test": {
		[DEEPKEY_DNA_NAME]:	DEEPKEY_DNA_PATH,
	    },
	}, {
	    "app_port": APP_PORT,
	    "actors": [
		"alice",
		"bobby",
	    ],
	});
    });

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function basic_tests () {
    let client;
    let app_client, bobby_client;
    let deepkey;
    let deepkey_csr;
    let ksr1_addr;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	app_client			= await client.app( "test-alice" );
	bobby_client			= await client.app( "test-bobby" );

	({
	    deepkey,
	}				= app_client.createInterface({
	    [DEEPKEY_DNA_NAME]:	DeepKeyCell,
	}));

	deepkey_csr			= deepkey.zomes.deepkey_csr.functions;

	await deepkey_csr.query_local_key_info();
    });

    it("should query keyset root action hash", async function () {
	await deepkey_csr.query_keyset_root_action_hash();
    });

    it("should query keyset authority action hash", async function () {
	ksr1_addr			= await deepkey_csr.query_keyset_authority_action_hash();
    });

    it("should query keyset members (1)", async function () {
	const members			= await deepkey_csr.query_keyset_members( ksr1_addr );
	log.normal("Keyset Root -> Members: %s", json.debug(members) );

	expect( members			).to.have.length( 1 );
    });

    it("should query keyset keys with authors (0)", async function () {
	const keys			= await deepkey_csr.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 0 );
    });

    it("should register new key", async function () {
	const registration_addr		= await deepkey_csr.register_key({
	    "app_name":		"Alice - App #1",
	    "dna_hash":		new DnaHash( crypto.randomBytes( 32 ) ),
	    "key":		new AgentPubKey( crypto.randomBytes( 32 ) ),
	    "signature":	crypto.randomBytes( 64 ),
	});
	log.normal("Key registration addr: %s", registration_addr );
    });

    it("should register new key (bobby)", async function () {
	const registration_addr		= new ActionHash( await bobby_client.call(
	    "deepkey", "deepkey_csr", "register_key", [
		new AgentPubKey( crypto.randomBytes( 32 ) ),
		crypto.randomBytes( 64 ),
		new DnaHash( crypto.randomBytes( 32 ) ),
		"Bobby - App #1",
	    ]
	) );
	log.normal("Key registration addr: %s", registration_addr );
    });

    it("should query keyset keys with authors (1)", async function () {
	const keys			= await deepkey_csr.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query keyset keys (1)", async function () {
	const keys			= await deepkey_csr.query_keyset_keys( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should get keyset root", async function () {
	const ksr			= await deepkey_csr.get_keyset_root( ksr1_addr );
	log.normal("Keyset Root: %s", json.debug(ksr) );
    });

    let invite_accept;
    it("should invite device", async function () {
	// const parent			= await deepkey_csr.query_keyset_authority_action_hash();
	// {
	//     "keyset_root":	ksr1_addr,
	//     "parent":		parent, // Either the KeysetRoot or the DeviceInviteAcceptance
	//     "invitee":		bobby_client.cell_client,
	// });
	invite_accept			= await deepkey_csr.invite_agent( bobby_client.agent_id );
	log.normal("Device Invite Acceptance: %s", json.debug(invite_accept) );
    });

    it("should accept invite", async function () {
	const acceptance_addr		= new ActionHash( await bobby_client.call(
	    "deepkey", "deepkey_csr", "accept_invite", invite_accept
	) );
	log.normal("Acceptance [addr]: %s", acceptance_addr );
    });

    it("should query keyset members (2)", async function () {
	const members			= await deepkey_csr.query_keyset_members( ksr1_addr );
	log.normal("Keyset Root -> Members: %s", json.debug(members) );

	expect( members			).to.have.length( 2 );
    });

    it("should query keyset keys (1)", async function () {
	const keys			= await deepkey_csr.query_keyset_keys( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    it("should query keyset keys with authors (1)", async function () {
	const keys			= await deepkey_csr.query_keyset_keys_with_authors( ksr1_addr );
	log.normal("Keyset Root -> Keys: %s", json.debug(keys) );

	expect( keys			).to.have.length( 1 );
    });

    linearSuite("Errors", function () {

	// it("should fail to create wasm entry because of wrong file size", async function () {
	//     await expect_reject(async () => {
	// 	await deepkey_csr.create_wasm_entry({
	// 	    "wasm_type": WASM_TYPES.INTEGRITY,
	// 	    "mere_memory_addr": wasm1.mere_memory_addr,
	// 	    "file_size": 0,
	// 	});
	//     }, "file size does not match memory address" );
	// });

    });

    after(async function () {
	await client.close();
    });
}