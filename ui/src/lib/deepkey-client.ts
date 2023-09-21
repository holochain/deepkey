import type { UnsubscribeFunction } from 'emittery';
import { isSignalFromCellWithRole } from '@holochain-open-dev/utils';
import type {
	ActionHash,
	AgentPubKey,
	AppAgentCallZomeRequest,
	AppAgentClient,
	AppSignal,
	DnaHash,
	HoloHashed,
	Signature
} from '@holochain/client';

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

export class DeepkeyClient {
	constructor(
		public client: AppAgentClient,
		public roleName: string,
		// public cellId: CellId,
		public zomeName = 'deepkey'
	) {}

	on<D>(listener: (eventData: D) => void | Promise<void>): UnsubscribeFunction {
		const eventName = 'signal'; // it's always 'signal'.
		return this.client.on(eventName, async (signal: AppSignal) => {
			if (
				(await isSignalFromCellWithRole(this.client, this.roleName, signal)) &&
				this.zomeName === signal.zome_name
			) {
				listener(signal.payload as D);
			}
		});
	}

	async key_state(keyAnchor: Array<number>): Promise<KeyState> {
		return this.callZome('key_state', [keyAnchor, Date.now()]);
	}

	async name_device(name: string): Promise<null> {
		return this.callZome('name_device', name);
	}

	async get_device_name(key: AgentPubKey): Promise<null> {
		return this.callZome('get_device_name', key);
	}

	async send_device_invitation(agent: AgentPubKey, dia: DeviceInviteAcceptance): Promise<null> {
		return this.callZome('send_device_invitation', [agent, dia]);
	}

	// Return the ActionHash of the Keyset Root
	async keyset_authority(): Promise<ActionHash> {
		return this.callZome('query_keyset_authority_action_hash', null);
	}

	// Take the ActionHash of the Keyset Root,
	// return the members of the Keyset by their AgentPubKey
	async query_keyset_members(ksr: ActionHash): Promise<AgentPubKey[]> {
		return this.callZome('query_keyset_members', ksr);
	}

	async query_keyset_keys(ksr: ActionHash): Promise<KeyRegistration[]> {
		return this.callZome('query_keyset_keys', ksr);
	}
	async query_keyset_key_anchors(ksr: ActionHash): Promise<KeyAnchor[]> {
		return this.callZome('query_keyset_key_anchors', ksr);
	}

	async invite_agent(agentKey: AgentPubKey): Promise<DeviceInviteAcceptance> {
		return this.callZome('invite_agent', agentKey);
	}
	async accept_invitation(dia: DeviceInviteAcceptance): Promise<ActionHash> {
		return this.callZome('accept_invite', dia);
	}

	// Returns the ActionHash of the created KeyRegistration
	async register_key(
		newKey: AgentPubKey,
		newKeySignature: Signature,
		dnaHash: DnaHash,
		appName: string
	): Promise<ActionHash> {
		return this.callZome('register_key', [newKey, newKeySignature, dnaHash, appName]);
	}

	// Returns the ActionHash of the created KeyRegistration
	async update_key(
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
	async revoke_key(
		keyRegistrationToRevoke: ActionHash,
		revocationAuthorization: [number, Signature][]
	): Promise<ActionHash> {
		return this.callZome('revoke_key', [keyRegistrationToRevoke, revocationAuthorization]);
	}

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	callZome(fn_name: string, payload: any) {
		const req: AppAgentCallZomeRequest = {
			role_name: this.roleName,
			// cell_id: this.cellId,
			zome_name: this.zomeName,
			fn_name,
			payload
		};
		return this.client.callZome(req, 30000);
	}
}
