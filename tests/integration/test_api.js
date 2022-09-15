const path				= require('path');
const log				= require('@whi/stdlog')(path.basename( __filename ), {
    level: process.env.LOG_LEVEL || 'fatal',
});


const fs				= require('fs');
const expect				= require('chai').expect;
const { expect_reject }                 = require('./utils.js');
const { HoloHash }			= require('@whi/holo-hash');
const { Holochain }			= require('@whi/holochain-backdrop');
const { ConductorError,
	EntryNotFoundError,
	DeserializationError,
	CustomError,
	...hc_client }			= require('@whi/holochain-client');

const json				= require('@whi/json');

const { backdrop }			= require('./setup.js');


const delay				= (n) => new Promise(f => setTimeout(f, n));
const DEEPKEY_DNA_PATH			= path.join(__dirname, "../../packs/deepkey.dna");
let clients;


function basic_tests () {
    it("should create a JoiningProof with a KeysetRoot", async function () {
	this.timeout( 10_000 );
	log.normal("Testing agent: %s", JSON.stringify( clients.alice ));
        let alice_agent			= new HoloHash( clients.alice._agent );
	log.normal(" - HoloHash:   %s", String(alice_agent))
	log.normal(" -  .toBytes:  %s", String(alice_agent.toBytes()))
	log.normal(" -  .getHash:  %s", String(alice_agent.getHash()))

	// Let's establish a new KeysetRoot, using alice's AgentPubKey
        /*
         * We'll generate one, instead 
         * 
	let keyset_root = {
	    first_deepkey_agent: clients.alice._agent,
	    root_pub_key: clients.alice._agent,  // Let's just re-use alice's pubkey TODO: create ephemeral
	    fda_pubkey_signed_by_root_key: Buffer.from("dc95af8774d5f3a94326e5e1d855c00a25abcb341deb453f211c58eaa964a6572f1bb3c2a9003cf1b363166616d8d3496da63856cca17ad9f13e62cd5ba7ec0c", "hex") // TODO: generate
	};
         * 
         */
        let keyset_root			= await clients.alice.call(
	    "deepkey-dna", "deepkey", "generate_keyset_root", [ null ]
	);
	log.normal("New KeysetRoot: %s", JSON.stringify( keyset_root ));

        /*
         * The default w/ None for spec_change is to authorize just this agent
         * to issue a new KeysetRoot.
         * 
	let new_spec = {
	    sigs_required: 1,
	    authorized_signers: [
		clients.alice._agent
	    ]
	}
	let spec_change = {
	    new_spec: new_spec,
	    authorization_of_new_spec: [
		Buffer.from("dc95af8774d5f3a94326e5e1d855c00a25abcb341deb453f211c58eaa964a6572f1bb3c2a9003cf1b363166616d8d3496da63856cca17ad9f13e62cd5ba7ec0c", "hex") // TODO: generate // clients.alice.sign( new_spec ) ??
	    ]
	};
        *
        */
	// First, initialize the Deepkey source-chain.  We'll include the KeysetRoot we're about to create.
        let addr			= new HoloHash( await clients.alice.call(
	    "deepkey-dna", "deepkey", "initialize", [ { KeysetRoot: keyset_root }, null ]
	));
	log.normal("New JoiningProof Action address: %s", String(addr) );

	// Second, create the KeysetRoot we just initialized with, and a ChangeRule.  This
	// KeysetRoot must be the first entry after JoiningProof.
        let create_keyset_args		= [ keyset_root, null, null ];
	log.normal("New KeysetRoot/Change: %s", JSON.stringify( create_keyset_args ));
        let addr_pair			= await clients.alice.call(
	    "deepkey-dna", "deepkey", "create_keyset_root", create_keyset_args
	);
	let addr_root			= new HoloHash( addr_pair[0] )
	let addr_chng			= new HoloHash( addr_pair[1] )
	log.normal("New KeysetRoot/Change Action addresses: %s, %s", String(addr_root), String(addr_chng) );

        // Invite another agent "Carol" to our newly created Deepkey group.  Provide a list of Agent IDs
        let carol_agent			= new HoloHash( clients.carol._agent );
        let invitees			= [ new Uint8Array(carol_agent.toBytes()), ]; //new Uint8Array(carol_agent.toBytes()); // 39-byte 
        log.normal("The invite_agents invitees: %s", String( invitees ));
        let acceptances			= await clients.alice.call(
            "deepkey-dna", "deepkey", "invite_agents", [ invitees ]
        );
        log.normal("The invite_agents Acceptances: %s", JSON.stringify( acceptances ));
    });
}

function errors_tests () {
    it("should fail creating an entry with known-invalid text", async function () {
        this.timeout( 10_000 )

        await expect_reject( async () => {
            await clients.alice.call( "deepkey-dna", "deepkey", "create", "invalid text" );
        }, ConductorError, "Source chain error: InvalidCommit error: invalid text" );
    });
}

describe("Zome: Deepkey", () => {

    const holochain			= new Holochain();

    before(async function () {
	this.timeout( 30_000 );

	clients				= await backdrop( holochain, {
	    "deepkey-dna": DEEPKEY_DNA_PATH,
	}, [
	    "alice", "carol"
	], {
	    "parse_entities": false,
	});
    });

    describe("Basic", basic_tests.bind( this, holochain ) );
    //describe("Errors", errors_tests.bind( this, holochain ) );

    after(async () => {
	await holochain.stop();
	await holochain.destroy();
    });

});