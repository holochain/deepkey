import type { UnsubscribeFunction } from 'emittery';
import type {
	ActionHash,
	AgentPubKey,
	AppAgentWebsocket,
	AppSignal,
	CellId,
	DnaHash,
	HoloHash,
	HoloHashed,
	Signature
} from '@holochain/client';
import { locallySignZomeCall } from './holochain-client';
import type { SelfAuthorizedHolochainWebsocket } from './store/holochain-client-store';
import { Base64 } from 'js-base64';

export type KeyAnchor = { bytes: Uint8Array };
export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
	return { bytes: pubkey.slice(3, 35) };
}

export type Authorization = [number, Buffer]; // u8 index, 64 byte signature

export type KeyGeneration = {
	new_key: AgentPubKey;
	new_key_signing_of_author: Uint8Array; // 64 byte signature
};

export type KeyRevocation = {
	prior_key_registration: ActionHash;
	revocation_authorization: Authorization[];
};

export type KeyRegistration =
	| {
			Create: KeyGeneration;
	  }
	| {
			CreateOnly: KeyGeneration;
	  }
	| {
			Update: [KeyRevocation, KeyGeneration];
	  }
	| {
			Delete: KeyRevocation;
	  };

export function getKeyFromKeyRegistration(registration: KeyRegistration): AgentPubKey | null {
	if ('Create' in registration) {
		const keyGen = registration.Create;
		return keyGen.new_key;
	} else if ('CreateOnly' in registration) {
		const keyGen = registration.CreateOnly;
		return keyGen.new_key;
	} else if ('Update' in registration) {
		const [keyRev, keyGen] = registration.Update;
		return keyGen.new_key;
	} else if ('Delete' in registration) {
		const keyRev = registration.Delete;
		return null;
	}
	return null;
}

export type KeyState =
	| {
			NotFound: null;
	  }
	| {
			Invalidated: { hashed: HoloHashed<KeyRegistration> };
	  }
	| {
			Valid: { hashed: HoloHashed<KeyRegistration> };
	  };

export type DeviceInviteAcceptance = {
	keyset_root_authority: ActionHash;
	invite: ActionHash;
};

export type DnaBinding = {
	key_meta: ActionHash; // Referencing a KeyMeta by its ActionHash
	dna_hash: HoloHash; //The hash of the DNA the key is bound to
	app_name: string;
};
export type KeyMeta = {
	new_key: ActionHash; // Referencing a KeyRegistration by its ActionHash
	derivation_path: Uint8Array;
	derivation_index: number;
	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	key_type: any;
};

export class DeepkeyClient {
	constructor(
		public client: SelfAuthorizedHolochainWebsocket | AppAgentWebsocket,
		// public roleName: string,
		public cellId: CellId,
		public zomeName = 'deepkey'
	) {}

	on<D>(listener: (eventData: D) => void | Promise<void>): UnsubscribeFunction {
		const eventName = 'signal'; // it's always 'signal'.
		return this.client.on(eventName, async (signal: AppSignal) => {
			if (
				// (await isSignalFromCellWithRole(this.client, this.roleName, signal)) &&
				cellIdsAreEqual(signal.cell_id, this.cellId) &&
				this.zomeName === signal.zome_name
			) {
				listener(signal.payload as D);
			}
		});
	}

	async keyState(keyAnchor: Array<number>): Promise<KeyState> {
		return this.callZome('key_state', [keyAnchor, Date.now()]);
	}

	async nameDevice(name: string): Promise<null> {
		return this.callZome('name_device', name);
	}

	async getDeviceName(key: AgentPubKey): Promise<null> {
		return this.callZome('get_device_name', key);
	}

	async sendDeviceInvitation(agent: AgentPubKey, dia: DeviceInviteAcceptance): Promise<null> {
		return this.callZome('send_device_invitation', [agent, dia]);
	}

	// Return the ActionHash of the Keyset Root
	async queryKeysetRoot(): Promise<ActionHash> {
		return this.callZome('query_keyset_authority_action_hash', null);
	}

	// Take the ActionHash of the Keyset Root,
	// return the members of the Keyset by their AgentPubKey
	async queryKeysetMembers(ksr: ActionHash): Promise<AgentPubKey[]> {
		return this.callZome('query_keyset_members', ksr);
	}

	async queryKeysetKeys(ksr: ActionHash): Promise<KeyRegistration[]> {
		return this.callZome('query_keyset_keys', ksr);
	}
	async queryKeysetKeysWithAuthors(ksr: ActionHash): Promise<Array<[AgentPubKey, KeyRegistration]>> {
		return this.callZome('query_keyset_keys_with_authors', ksr);
	}
	async queryKeysetKeyAnchors(ksr: ActionHash): Promise<KeyAnchor[]> {
		return this.callZome('query_keyset_key_anchors', ksr);
	}

	async inviteAgent(agentKey: AgentPubKey): Promise<DeviceInviteAcceptance> {
		return this.callZome('invite_agent', agentKey);
	}
	async acceptInvitation(dia: DeviceInviteAcceptance): Promise<ActionHash> {
		return this.callZome('accept_invite', dia);
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	async queryLocalKeyInfo(): Promise<any> {
		// Array<[DnaBinding, KeyMeta, KeyRegistration]>
		// return this.callZome('name_device', 'Test');

		return this.callZome('query_local_key_info', null);
	}

	// Returns the ActionHash of the created KeyRegistration
	async registerKey(
		newKey: AgentPubKey,
		newKeySignature: Signature,
		dnaHash: DnaHash,
		appName: string
	): Promise<ActionHash> {
		return this.callZome('register_key', [newKey, newKeySignature, dnaHash, appName]);
	}

	// Returns the ActionHash of the created KeyRegistration
	async updateKey(
		priorKeyRegistration: ActionHash,
		revocationAuthorization: [number, Signature][],
		newKey: AgentPubKey,
		newKeySignature: Signature,
		dnaHash: DnaHash,
		appName: string
	): Promise<ActionHash> {
		return this.callZome('update_key', [newKey, newKeySignature, dnaHash, appName]);
	}

	// Returns the ActionHash of the created KeyRegistration
	async revokeKey(
		keyRegistrationToRevoke: ActionHash,
		revocationAuthorization: [number, Signature][]
	): Promise<ActionHash> {
		return this.callZome('revoke_key', [keyRegistrationToRevoke, revocationAuthorization]);
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	async callZome(fn_name: string, payload: any) {
		let req = {
			cell_id: this.cellId,
			zome_name: this.zomeName,
			fn_name,
			payload
		};

		if ('creds' in this.client) {
			// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
			const creds = this.client.creds!;
			req = await locallySignZomeCall({ ...req, provenance: creds.signingKey }, creds);
		}

		console.log('deepkey callZome!', fn_name);
		return this.client.callZome(req, 30000);
	}
}

function cellIdsAreEqual(cell1: CellId, cell2: CellId): boolean {
	return (
		Base64.fromUint8Array(new Uint8Array([...cell1[0], ...cell1[1]])) ===
		Base64.fromUint8Array(new Uint8Array([...cell2[0], ...cell2[1]]))
	);
}
