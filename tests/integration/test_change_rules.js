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
const revocation_key3			= ed.utils.randomPrivateKey();
const revocation_key4			= ed.utils.randomPrivateKey();

const rev1_pubkey			= await ed.getPublicKeyAsync( revocation_key1 );
const rev2_pubkey			= await ed.getPublicKeyAsync( revocation_key2 );
const rev3_pubkey			= await ed.getPublicKeyAsync( revocation_key3 );
const rev4_pubkey			= await ed.getPublicKeyAsync( revocation_key4 );

let APP_PORT;

describe("DeepKey", function () {
    const holochain			= new Holochain({
	"timeout": 10_000,
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

    linearSuite("Change Rules", basic_tests );

    after(async () => {
	await holochain.destroy();
    });
});


function basic_tests () {
    let client;
    let alice1_client, alice2_client;
    let deepkey;
    let alice1_deepkey;
    let alice2_deepkey;
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

    it("should update change rule for (alice1) KSR", async function () {
	const auth_spec_package		= await alice1_deepkey.construct_authority_spec({
	    "sigs_required": 2,
	    "authorized_signers": [
		rev1_pubkey,
		rev2_pubkey,
		rev3_pubkey,
	    ],
	});
	// log.normal("Constructed Authority Spec: %s", json.debug(auth_spec_package.authority_spec) );

	const new_change_rule		= await alice1_deepkey.update_change_rule({
	    "authority_spec": auth_spec_package.authority_spec,
	});

	log.normal("New Change Rule: %s", json.debug(new_change_rule) );
    });

    it("should update change rule using revocation key for (alice1) KSR", async function () {
	const auth_spec_package		= await alice1_deepkey.construct_authority_spec({
	    "sigs_required": 2,
	    "authorized_signers": [
		rev1_pubkey,
		rev2_pubkey,
		rev3_pubkey,
		rev4_pubkey,
	    ],
	});
	// log.normal("Constructed Authority Spec: %s", json.debug(auth_spec_package.authority_spec) );

	log.info("Signing new auth spec with authority: %s", new AgentPubKey( rev1_pubkey ) );
	const new_change_rule		= await alice1_deepkey.update_change_rule({
	    "authority_spec": auth_spec_package.authority_spec,
	    "authorizations": [
		[ 0, await ed.signAsync( auth_spec_package.serialized, revocation_key1 ) ],
		[ 2, await ed.signAsync( auth_spec_package.serialized, revocation_key3 ) ],
	    ],
	});

	log.normal("New Change Rule: %s", json.debug(new_change_rule) );

	const current_change_rule	= await alice1_deepkey.get_current_change_rule_for_ksr( ksr1_addr );

	expect( current_change_rule	).to.deep.equal( new_change_rule );
    });

    linearSuite("Errors (pre invite acceptance)", function () {

	it("should fail to update change rule entry because invalid signature for (alice1) KSR", async function () {
	    await expect_reject(async () => {
		await alice1_deepkey.update_change_rule({
		    "authority_spec": {
			"sigs_required": 1,
			"authorized_signers": [
			    crypto.randomBytes(32),
			],
		    },
		    "authorizations": [
			[ 0, crypto.randomBytes(64) ],
			[ 1, crypto.randomBytes(64) ],
		    ],
		});
	    }, "Authorization has invalid signature" );
	});

	it("should fail to update change rule entry because invalid signature for (alice1) KSR", async function () {
	    await expect_reject(async () => {
		await alice1_deepkey.update_change_rule({
		    "authority_spec": {
			"sigs_required": 2,
			"authorized_signers": [
			    crypto.randomBytes(32),
			],
		    },
		    "authorizations": [
			[ 0, crypto.randomBytes(64) ],
		    ],
		});
	    }, "There are not enough authorities" );
	});

	it("should fail to update change rule entry because not enough signatures for (alice1) KSR", async function () {
	    await expect_reject(async () => {
		const auth_spec_package		= await alice1_deepkey.construct_authority_spec({
		    "sigs_required": 1,
		    "authorized_signers": [
			crypto.randomBytes(32),
		    ],
		});
		await alice1_deepkey.update_change_rule({
		    "authority_spec": {
			"sigs_required": 1,
			"authorized_signers": [
			    crypto.randomBytes(32),
			],
		    },
		    "authorizations": [
			[ 0, await ed.signAsync( auth_spec_package.serialized, revocation_key1 ) ],
		    ],
		});
	    }, "change rule requires at least" );
	});

	it("should fail to update change rule entry because invalid signature for (alice1) KSR", async function () {
	    await expect_reject(async () => {
		const auth_spec_package		= await alice1_deepkey.construct_authority_spec({
		    "sigs_required": 0,
		    "authorized_signers": [
			crypto.randomBytes(32),
		    ],
		});
		await alice1_deepkey.update_change_rule({
		    "authority_spec": auth_spec_package.authority_spec,
		    "authorizations": [
			[ 0, await ed.signAsync( auth_spec_package.serialized, revocation_key1 ) ],
			[ 2, await ed.signAsync( auth_spec_package.serialized, revocation_key3 ) ],
		    ],
		});
	    }, "Required signatures cannot be 0" );
	});

	linearSuite("Phase 2", phase2 );
    });

    function phase2 () {
	let invite_accept;

	it("(alice2) should invite device 'alice1'", async function () {
	    this.timeout( 5_000 );

	    invite_accept			= await alice2_deepkey.invite_agent( alice2_client.agent_id );
	    log.normal("Device Invite Acceptance: %s", json.debug(invite_accept) );
	});

	it("(alice1) should accept invite from 'alice2'", async function () {
	    this.timeout( 5_000 );

	    const acceptance_addr		= await alice1_deepkey.accept_invite( invite_accept );
	    log.normal("Acceptance [addr]: %s", acceptance_addr );
	});

	linearSuite("Errors (post invite acceptance)", function () {

	    it("should fail to update change rule because of invite acceptance for (alice1) KSR", async function () {
		await expect_reject(async () => {
		    await alice1_deepkey.update_change_rule({
			"authority_spec": {
			    "sigs_required": 1,
			    "authorized_signers": [
				crypto.randomBytes(32),
			    ],
			},
			"authorizations": [
			    [ 0, crypto.randomBytes(64) ],
			],
		    });
		}, "Cannot change rules for KSR because a Device Invite was accepted" );
	    });

	});
    }

    after(async function () {
	await client.close();
    });
}
