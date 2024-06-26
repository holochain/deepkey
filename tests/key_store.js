import { Logger }                       from '@whi/weblogger';
const log                               = new Logger("key-store", process.env.LOG_LEVEL );

import crypto                           from 'crypto';
import * as ed                          from '@noble/ed25519';
import { hmac }                         from '@noble/hashes/hmac';
import { sha256 }                       from '@noble/hashes/sha256';

import {
    AgentPubKey,
}                                       from '@spartan-hc/holo-hash';


export class KeyStore {
    #device_seed                        = null;
    #name                               = Buffer.from( crypto.randomBytes( 12 ) ).toString("hex");
    #keys                               = {};

    constructor ( device_seed, name ) {
        this.#device_seed               = Buffer.from( device_seed ).toString("hex");

        if ( name )
            this.#name                  = name;
    }

    get name () {
        return this.#name;
    }

    get seed () {
        return this.#device_seed;
    }

    async createKey ( path ) {
        if ( typeof path !== "string" )
            throw new TypeError(`Path must be a string; not type '${typeof path}'`);

        const secret                    = hmac( sha256, this.#device_seed, path );
        const key                       = new Key( secret, Buffer.from(path, "utf8") )
        const agent                     = await key.getAgent();

        log.normal("[%s] Created key from derivation path '%s': (agent) %s", this.name, path, agent );
        this.#keys[agent]               = key;

        return key;
    }

    getKey ( agent ) {
        return this.#keys[ new AgentPubKey( agent ) ];
    }
}


export class Key {
    #secret                             = null;
    #bytes                              = null;
    #derivation_bytes                   = null;

    constructor ( secret, derivation_bytes ) {
        if ( !(secret instanceof Uint8Array) )
            throw new TypeError(`Secret must be a Uint8Array; not type '${secret?.constructor?.name || typeof secret}'`);
        if ( secret.length !== 32 )
            throw new Error(`Secret must 32 bytes; not length ${secret.length}`);

        this.#secret                    = secret;
        this.#derivation_bytes          = derivation_bytes;
    }

    get derivation_bytes () {
        return this.#derivation_bytes;
    }

    async getBytes () {
        if ( this.#bytes === null )
            this.#bytes                 = await ed.getPublicKeyAsync( this.#secret );

        return new Uint8Array( this.#bytes );
    }

    async getAgent () {
        return new AgentPubKey( await this.getBytes() );
    }

    async sign ( bytes ) {
        if ( !(bytes instanceof Uint8Array) )
            throw new TypeError(`Key signing expects a Uint8Array; not type '${bytes?.constructor?.name || typeof bytes}'`);

        return await ed.signAsync( bytes, this.#secret );
    }
}


export function random_key () {
    const secret                        = ed.utils.randomPrivateKey();
    return new Key( secret );
}


export default {
    KeyStore,
    Key,
    random_key,
};
