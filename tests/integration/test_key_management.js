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
    random_key,
}                                       from '../key_store.js';


const __dirname                         = path.dirname( new URL(import.meta.url).pathname );
const DEEPKEY_DNA_PATH                  = path.join( __dirname, "../../dnas/deepkey.dna" );
const DEEPKEY_DNA_NAME                  = "deepkey";

const dna1_hash                         = new DnaHash( crypto.randomBytes( 32 ) );

const ALICE_DEVICE_SEED                 = Buffer.from("jJQhp80zPT+XBMOZmtfwdBqY9ay9k2w520iwaet1if4=", "base64");
const BOBBY_DEVICE_SEED                 = Buffer.from("jJQhp80zPT+XBMOZmtfwdBqY9ay9k2w520iwaet1if4=", "base64");

const alice_key_store                   = new KeyStore( ALICE_DEVICE_SEED, "alice" );
const bobby_key_store                   = new KeyStore( BOBBY_DEVICE_SEED, "bobby" );

const alice_app1_id                     = "alice-app1";
const alice_app2_id                     = "alice-app2";
const alice_app3_id                     = "alice-app3";
const bobby_app1_id                     = "bobby-app1";

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
            "alice",
            "bobby",
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

    linearSuite("Key Management", basic_tests );

    after(async () => {
        await holochain.destroy();
    });
});


function basic_tests () {
    let client;
    let alice_client;
    let bobby_client;
    let deepkey;
    let alice_deepkey;
    let bobby_deepkey;
    let ksr1_addr;

    let alice_key1a_reg, alice_key1a_reg_addr, alice_key1a;
    let alice_key1b_reg, alice_key1b_reg_addr, alice_key1b;
    let alice_key1c_reg, alice_key1c_reg_addr, alice_key1c;
    let alice_key1d_reg, alice_key1d_reg_addr;
    let alice_key2a_reg, alice_key2a_reg_addr, alice_key2a;
    let alice_key3a_reg, alice_key3a_reg_addr, alice_key3a, alice_key3;

    before(async function () {
        this.timeout( 30_000 );

        client                          = new AppInterfaceClient( app_port, {
            "logging": process.env.LOG_LEVEL || "normal",
        });

        const alice_token               = installations.alice.test.auth.token;
        alice_client                    = await client.app( alice_token );

        const bobby_token               = installations.bobby.test.auth.token;
        bobby_client                    = await client.app( bobby_token );

        {
            ({
                deepkey,
            }                           = alice_client.createInterface({
                [DEEPKEY_DNA_NAME]:     DeepKeyCell,
            }));

            alice_deepkey               = deepkey.zomes.deepkey_csr.functions;
        }

        {
            ({
                deepkey,
            }                           = bobby_client.createInterface({
                [DEEPKEY_DNA_NAME]:     DeepKeyCell,
            }));

            bobby_deepkey               = deepkey.zomes.deepkey_csr.functions;
        }

        ksr1_addr                       = await alice_deepkey.query_keyset_authority_action_hash();
    });

    it("should register new key (alice)", async function () {
        this.timeout( 5_000 );

        const app_index                 = next_app_index();
        const key_index                 = 0;
        const path                      = `app/${app_index}/key/${key_index}`;
        const new_key                   = await alice_key_store.createKey( path );

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice - App #1",
                "installed_app_id":     alice_app1_id,
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "derivation_details":       {
                app_index,
                key_index,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (create) addr: %s", addr );

        alice_key1a                     = await new_key.getBytes();
        alice_key1a_reg                 = key_reg;
        alice_key1a_reg_addr            = addr;

        {
            const key_state             = await alice_deepkey.key_state( alice_key1a );
            log.normal("Key (1a) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Valid" );
        }
    });

    it("should query (alice) keyset keys (1)", async function () {
        const keys                      = await alice_deepkey.query_apps_with_keys();
        log.normal("Keyset app keys: %s", json.debug(keys) );

        expect( keys                    ).to.have.length( 2 );
    });

    it("should update key (alice)", async function () {
        this.timeout( 5_000 );

        const derivation_details        = await alice_deepkey.next_derivation_details( alice_key1a );
        const {
            app_index,
            key_index,
        }                               = derivation_details;
        const path                      = `app/${app_index}/key/${key_index}`;
        const new_key                   = await alice_key_store.createKey( path );

        expect( derivation_details      ).to.deep.equal({
            "app_index": 1,
            "key_index": 1,
        });

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.update_key({
            "key_revocation": {
                "prior_key_registration": alice_key1a_reg_addr,
                "revocation_authorization": [
                    [ 0, await alice_deepkey.sign( alice_key1a_reg_addr ) ],
                ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "derivation_details":       {
                ...derivation_details,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (update) addr: %s", addr );

        alice_key1b                     = await new_key.getBytes();
        alice_key1b_reg                 = key_reg;
        alice_key1b_reg_addr            = addr;

        expect( key_meta.key_index      ).to.equal( 1 );

        {
            const key_state             = await alice_deepkey.key_state( alice_key1a );
            log.normal("Key (1a) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Invalid" );
        }
        {
            const key_state             = await alice_deepkey.key_state( alice_key1b );
            log.normal("Key (1b) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Valid" );
        }
    });

    it("should query (alice) keyset keys (1)", async function () {
        const keys                      = await alice_deepkey.query_apps_with_keys();
        log.normal("Keyset app keys: %s", json.debug(keys) );

        expect( keys                    ).to.have.length( 2 );
    });

    it("should get (alice) KSR keys (1)", async function () {
        const keys                      = await alice_deepkey.get_ksr_keys( ksr1_addr );
        log.normal("KSR keys: %s", json.debug(keys) );

        expect( keys                    ).to.have.length( 2 );
    });

    it("should query (alice) app bindings", async function () {
        let app_bindings                        = await alice_deepkey.query_app_bindings();
        log.normal("App Bindings: %s", json.debug(app_bindings) );

        expect( app_bindings            ).to.have.length( 2 );
    });

    it("should update key (alice)", async function () {
        this.timeout( 5_000 );

        const derivation_details        = await alice_deepkey.next_derivation_details( alice_key1b );
        const {
            app_index,
            key_index,
        }                               = derivation_details;
        const path                      = `app/${app_index}/key/${key_index}`;
        const new_key                   = await alice_key_store.createKey( path );

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.update_key({
            "key_revocation": {
                "prior_key_registration": alice_key1b_reg_addr,
                "revocation_authorization": [
                    [ 0, await alice_deepkey.sign( alice_key1b_reg_addr ) ],
                ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "derivation_details":       {
                ...derivation_details,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (update) addr: %s", addr );

        alice_key1c                     = await new_key.getBytes();
        alice_key1c_reg                 = key_reg;
        alice_key1c_reg_addr            = addr;
    });

    it("should revoke key (alice)", async function () {
        this.timeout( 5_000 );

        const [ addr, key_reg ] = await alice_deepkey.revoke_key({
            "key_revocation": {
                "prior_key_registration": alice_key1c_reg_addr,
                "revocation_authorization": [
                    [ 0, await alice_deepkey.sign( alice_key1c_reg_addr ) ],
                ],
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key registration (update) addr: %s", addr );

        alice_key1d_reg                 = key_reg;
        alice_key1d_reg_addr            = addr;

        {
            const key_state             = await alice_deepkey.key_state( alice_key1a );
            log.normal("Key (1a) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Invalid" );
            expect( key_state.Invalid   ).to.not.be.null;
        }
        {
            const key_state             = await alice_deepkey.key_state( alice_key1b );
            log.normal("Key (1b) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Invalid" );
            expect( key_state.Invalid   ).to.not.be.null;
        }
        {
            const key_state             = await alice_deepkey.key_state( alice_key1c );
            log.normal("Key (1c) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Invalid" );
            expect( key_state.Invalid   ).to.not.be.null;
        }
    });

    it("should check key state before creation (alice)", async function () {
        const timestamp                 = Date.now() - (60 * 60 * 1000); // 1 hour ago
        const key_state                 = await alice_deepkey.key_state([ alice_key1a, timestamp ]);
        log.normal("Key (1a) state @ %s: %s", timestamp, json.debug(key_state) );

        expect( key_state               ).to.have.key( "Invalid" );
        expect( key_state.Invalid       ).to.be.null;
    });

    it("should register another key", async function () {
        this.timeout( 10_000 );

        const app_index                 = next_app_index();
        const key_index                 = 0;
        const path                              = `app/${app_index}/key/${key_index}`;
        const new_key                           = await alice_key_store.createKey( path );

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice - App #2",
                "installed_app_id":     alice_app2_id,
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "derivation_details":       {
                app_index,
                key_index,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });

        alice_key2a                     = await new_key.getBytes();
        alice_key2a_reg                 = key_reg;
        alice_key2a_reg_addr            = addr;
    });

    it("should register new key with the same app ID (alice)", async function () {
        this.timeout( 5_000 );

        next_app_index(); // Skip a number

        const app_index                 = next_app_index();
        const key_index                 = 0;
        const path                      = `app/${app_index}/key/${key_index}`;
        const new_key                   = await alice_key_store.createKey( path );

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice - App #1",
                "installed_app_id":     alice_app1_id,
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "derivation_details":       {
                app_index,
                key_index,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (create) addr: %s", addr );
    });

    it("should get derivation details for key 1b (alice)", async function () {
        {
            const derivation_details    = await alice_deepkey.get_key_derivation_details( alice_client.agent_id.getHash() );

            log.normal("Key (0a) derivation details: %s", json.debug(derivation_details) );

            expect( derivation_details  ).to.deep.equal({
                app_index: 0,
                key_index: 0,
            });
        }
        {
            const derivation_details    = await alice_deepkey.get_key_derivation_details( alice_key1b );

            log.normal("Key (1b) derivation details: %s", json.debug(derivation_details) );

            expect( derivation_details  ).to.deep.equal({
                app_index: 1,
                key_index: 1,
            });
        }
        {
            const derivation_details    = await alice_deepkey.get_key_derivation_details( alice_key2a );

            log.normal("Key (2a) derivation details: %s", json.debug(derivation_details) );

            expect( derivation_details  ).to.deep.equal({
                app_index: 2,
                key_index: 0,
            });
        }
    });

    it("should register unmanaged key (alice)", async function () {
        this.timeout( 5_000 );

        const new_key                   = await random_key();

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice - App #3",
                "installed_app_id":     alice_app3_id,
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
            "create_only": true,
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (create) addr: %s", addr );

        alice_key3                      = new_key;
        alice_key3a                     = await new_key.getBytes();
        alice_key3a_reg                 = key_reg;
        alice_key3a_reg_addr            = addr;

        {
            const key_state             = await alice_deepkey.key_state( alice_key3a );
            log.normal("Key (3a) state: %s", json.debug(key_state) );

            expect( key_state           ).to.have.key( "Valid" );
        }
    });

    it("should register new key without derivation details", async function () {
        this.timeout( 5_000 );

        const new_key                   = await random_key();

        const [ addr, key_reg, key_meta ]       = await alice_deepkey.create_key({
            "app_binding": {
                "app_name":             "Alice - App #4",
                "installed_app_id":     "random_agent_key",
                "dna_hashes":           [ dna1_hash ],
            },
            "key_generation": {
                "new_key":                      await new_key.getAgent(),
                "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
            },
        });
        log.normal("Key Registration (%s): %s", addr, json.debug(key_reg) );
        log.normal("Key Meta: %s", json.debug(key_meta) );
        log.normal("Key registration (update) addr: %s", addr );
    });

    it("should query key lineage", async function () {
        {
            const lineage               = await alice_deepkey.query_key_lineage( alice_key1a );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
        {
            const lineage               = await alice_deepkey.query_key_lineage( alice_key1b );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
        {
            const lineage               = await alice_deepkey.query_key_lineage( alice_key1c );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
    });

    it("should get key lineage", async function () {
        {
            const lineage               = await alice_deepkey.get_key_lineage( alice_key1a );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
        {
            const lineage               = await alice_deepkey.get_key_lineage( alice_key1b );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
        {
            const lineage               = await alice_deepkey.get_key_lineage( alice_key1c );
            log.normal("Key lineage: %s", json.debug(lineage) );

            expect( lineage             ).to.have.length( 3 );
            expect( lineage[0]          ).to.deep.equal( alice_key1a );
            expect( lineage[1]          ).to.deep.equal( alice_key1b );
            expect( lineage[2]          ).to.deep.equal( alice_key1c );
        }
    });

    it("should check keys have the same lineage", async function () {
        // Query
        {
            const same_lineage          = await alice_deepkey.query_same_lineage([
                alice_key1a,
                alice_key1c,
            ]);
            log.normal("Keys have the same lineage: %s", same_lineage );

            expect( same_lineage        ).to.be.true;
        }
        {
            const same_lineage          = await alice_deepkey.query_same_lineage([
                alice_key1a,
                alice_key2a,
            ]);
            log.normal("Keys have the same lineage: %s", same_lineage );

            expect( same_lineage        ).to.be.false;
        }
        // Get
        {
            const same_lineage          = await alice_deepkey.same_lineage([
                alice_key1a,
                alice_key1c,
            ]);
            log.normal("Keys have the same lineage: %s", same_lineage );

            expect( same_lineage        ).to.be.true;
        }
        {
            const same_lineage          = await alice_deepkey.same_lineage([
                alice_key1a,
                alice_key2a,
            ]);
            log.normal("Keys have the same lineage: %s", same_lineage );

            expect( same_lineage        ).to.be.false;
        }
    });

    it("should check existing derivation details", async function () {
        this.timeout( 10_000 );

        {
            const app_index             = 1;
            const key_index             = 0;
            const path                  = `app/${app_index}/key/${key_index}`;
            const new_key               = await alice_key_store.createKey( path );

            const { key_meta }          = await alice_deepkey.check_existing_derivation_details({
                app_index,
                key_index,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     new_key.derivation_bytes,
            });

            log.normal("Key meta: %s", json.debug(key_meta) );
            expect( key_meta.derivation_bytes   ).to.deep.equal( new_key.derivation_bytes );
        }

        {
            const app_index             = 1;
            const key_index             = 0;
            const path                  = `app/${app_index}/key/${key_index}`;
            const new_key               = await alice_key_store.createKey( path );

            const {
                app_binding,
                key_meta,
            }                           = await alice_deepkey.check_existing_derivation_details({
                app_index,
                key_index,
                "derivation_seed":      alice_key_store.seed,
                "derivation_bytes":     [0,0,0],
            });

            log.normal("Key meta: %s", json.debug(key_meta) );

            expect( app_binding.app_index       ).to.equal( app_index );
            expect( key_meta.key_index          ).to.equal( key_index );
            expect( key_meta.derivation_seed    ).to.deep.equal( alice_key_store.seed );
        }
    });

    linearSuite("Errors", function () {

        it("should fail to register invalid key", async function () {
            await expect_reject(async () => {
                await alice_deepkey.create_key({
                    "app_binding": {
                        "app_name":             "?",
                        "installed_app_id":     "?",
                        "dna_hashes":           [ dna1_hash ],
                    },
                    "key_generation": {
                        "new_key":                      new AgentPubKey( crypto.randomBytes( 32 ) ),
                        "new_key_signing_of_author":    crypto.randomBytes( 64 ),
                    },
                });
            }, "Signature does not match new key" );
        });

        it("should fail to manage CreateOnly key", async function () {
            this.timeout( 10_000 );

            await expect_reject(async () => {
                await alice_deepkey.revoke_key({
                    "key_revocation": {
                        "prior_key_registration": alice_key3a_reg_addr,
                        "revocation_authorization": [
                            [ 0, await alice_key3.sign( alice_key3a_reg_addr ) ],
                        ],
                    },
                });
            }, "cannot be updated" );
        });

        it("should fail to revoke key", async function () {
            this.timeout( 10_000 );

            await expect_reject(async () => {
                await alice_deepkey.revoke_key({
                    "key_revocation": {
                        "prior_key_registration": alice_key2a_reg_addr,
                        "revocation_authorization": [
                            [ 0, crypto.randomBytes(64) ],
                        ],
                    },
                });
            }, "Authorization has invalid signature" );
        });

        it("should fail to revoke key that belongs to another KSR", async function () {
            this.timeout( 10_000 );

            await expect_reject(async () => {
                await bobby_deepkey.delete_key_registration([
                    alice_key2a_reg_addr,
                    {
                        "prior_key_registration": alice_key2a_reg_addr,
                        "revocation_authorization": [
                            [ 0, await alice_deepkey.sign( alice_key2a_reg_addr ) ],
                        ],
                    },
                ]);
            }, "cannot revoke key registered by another author" );
        });

        it("should fail to duplicate a revoke", async function () {
            this.timeout( 10_000 );

            await expect_reject(async () => {
                await alice_deepkey.revoke_key({
                    "key_revocation": {
                        "prior_key_registration": alice_key1c_reg_addr,
                        "revocation_authorization": [
                            [ 0, await alice_deepkey.sign( alice_key1c_reg_addr ) ],
                        ],
                    },
                });
            }, "already a KeyRegistration" );
        });

        it("should fail to update key because app_index changed", async function () {
            await expect_reject(async () => {
                const app_index             = 0;
                const {
                    key_index,
                }                           = await alice_deepkey.next_derivation_details( alice_key1c );
                const path                  = `app/${app_index}/key/${key_index}`;
                const new_key               = await alice_key_store.createKey( path );

                await alice_deepkey.update_key({
                    "key_revocation": {
                        "prior_key_registration": alice_key1c_reg_addr,
                        "revocation_authorization": [
                            [ 0, await alice_deepkey.sign( alice_key1c_reg_addr ) ],
                        ],
                    },
                    "key_generation": {
                        "new_key":                      await new_key.getAgent(),
                        "new_key_signing_of_author":    await new_key.sign( alice_client.agent_id ),
                    },
                    "derivation_details":       {
                        app_index,
                        key_index,
                        "derivation_seed":      alice_key_store.seed,
                        "derivation_bytes":     new_key.derivation_bytes,
                    },
                });
            }, "derivation app index does not match the app binding" );
        });

    });

    after(async function () {
        await client.close();
    });
}
