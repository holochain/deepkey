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

    linearSuite("Basic", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function basic_tests () {
    let client;
    let alice1_client;
    let deepkey;
    let deepkey_csr;
    let ksr1_addr;

    before(async function () {
	this.timeout( 30_000 );

	client				= new AppInterfaceClient( APP_PORT, {
	    "logging": process.env.LOG_LEVEL || "normal",
	});
	alice1_client			= await client.app( "test-alice1" );

	({
	    deepkey,
	}				= alice1_client.createInterface({
	    [DEEPKEY_DNA_NAME]:	DeepKeyCell,
	}));

	deepkey_csr			= deepkey.zomes.deepkey_csr.functions;

	ksr1_addr			= await deepkey_csr.query_keyset_authority_action_hash();
    });

    it("should query keyset root action hash", async function () {
	await deepkey_csr.query_keyset_root_action_hash();
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
	const secret			= ed.utils.randomPrivateKey();
	const pubkey_bytes		= await ed.getPublicKeyAsync( secret );

	const registration_addr		= await deepkey_csr.register_key({
	    "app_name":		"Alice1 - App #1",
	    "dna_hashes":	[ dna1_hash ],
	    "key":		new AgentPubKey( pubkey_bytes ),
	    "signature":	await ed.signAsync( alice1_client.agent_id, secret ),
	});
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

    it("should query local key info", async function () {
	let key_info			= await deepkey_csr.query_key_info();
	log.normal("Key info: %s", json.debug(key_info) );

	expect( key_info		).to.have.length( 1 );
    });

    after(async function () {
	await client.close();
    });
}
