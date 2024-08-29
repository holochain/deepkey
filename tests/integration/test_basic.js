import { Logger }                       from '@whi/weblogger';
const log                               = new Logger("test-basic", process.env.LOG_LEVEL );

// import why                           from 'why-is-node-running';

import path                             from 'path';
import crypto                           from 'crypto';

import { expect }                       from 'chai';

import json                             from '@whi/json';
import {
    HoloHash,
    DnaHash, AgentPubKey,
    ActionHash, EntryHash,
}                                       from '@spartan-hc/holo-hash';
import { Holochain }                    from '@spartan-hc/holochain-backdrop';

import {
    DeepKeyCell,
}                                       from '@holochain/deepkey-zomelets';
import {
    AppInterfaceClient,
}                                       from '@spartan-hc/app-interface-client';

import {
    expect_reject,
    linearSuite,
}                                       from '../utils.js';
import {
    KeyStore,
}                                       from '../key_store.js';


const __dirname                         = path.dirname( new URL(import.meta.url).pathname );
const DEEPKEY_DNA_PATH                  = path.join( __dirname, "../../dnas/deepkey.dna" );
const DEEPKEY_DNA_NAME                  = "deepkey";

const dna1_hash                         = new DnaHash( crypto.randomBytes( 32 ) );

const ALICE1_DEVICE_SEED                = Buffer.from("jJQhp80zPT+XBMOZmtfwdBqY9ay9k2w520iwaet1if4=", "base64");

const alice1_app1_id                    = "alice1-app1";
const alice1_key_store                  = new KeyStore( ALICE1_DEVICE_SEED, "alice1" );

let app_port;
let installations;
let app_count                           = 1;

function next_app_index () {
    return app_count++;
}


describe("DeepKey", function () {
    const holochain                     = new Holochain({
        "timeout": 20_000,
        "default_stdout_loggers": log.level_rank > 3,
    });

    before(async function () {
        this.timeout( 60_000 );

        installations                   = await holochain.install([
            "alice1",
        ], {
            "app_name": "test",
            "bundle": {
                [DEEPKEY_DNA_NAME]:     DEEPKEY_DNA_PATH,
            },
            "membrane_proofs": {
                [DEEPKEY_DNA_NAME]:     {
                    "joining_proof":    crypto.randomBytes( 32 ),
                },
            },
        });

        app_port                        = await holochain.ensureAppPort();
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
    let alice1_deepkey;
    let ksr1_addr;

    before(async function () {
        this.timeout( 30_000 );

        client                          = new AppInterfaceClient( app_port, {
            "logging": process.env.LOG_LEVEL || "normal",
        });

        const alice1_token              = installations.alice1.test.auth.token;
        alice1_client                   = await client.app( alice1_token );

        ({
            deepkey,
        }                               = alice1_client.createInterface({
            [DEEPKEY_DNA_NAME]: DeepKeyCell,
        }));

        alice1_deepkey                  = deepkey.zomes.deepkey_csr.functions;

        ksr1_addr                       = await alice1_deepkey.query_keyset_authority_action_hash();
    });

    it("should query keyset root action hash", async function () {
        await alice1_deepkey.query_keyset_root_action_hash();
    });

    it("should get KSR keys (1)", async function () {
        const keys                      = await alice1_deepkey.get_ksr_keys( ksr1_addr );
        log.normal("KSR keys: %s", json.debug(keys) );

        expect( keys                    ).to.have.length( 1 );
    });

    it("should register new key", async function () {
        this.timeout( 5_000 );

        const app_index                 = next_app_index();
        const key_index                 = 0;
        const path                      = `app/${app_index}/key/${key_index}`;
        const new_key                   = await alice1_key_store.createKey( path );

        const [ addr, key_reg, key_meta ]       = await alice1_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice1 - App #1",
                "installed_app_id":     "alice1-app1",
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice1_client.agent_id ),
            },
            "derivation_details":       {
                app_index,
                key_index,
                "derivation_seed":      alice1_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (update) addr: %s", addr );
    });

    it("should get KSR keys (2)", async function () {
        const keys                      = await alice1_deepkey.get_ksr_keys( ksr1_addr );
        log.normal("KSR keys: %s", json.debug(keys) );

        expect( keys                    ).to.have.length( 2 );
    });

    after(async function () {
        await client.close();
    });
}
