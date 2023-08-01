// import type {
// 	ActionHash,
// 	AgentPubKey,
// 	AppAgentCallZomeRequest,
// 	AppAgentClient,
// 	EntryHash,
// 	HoloHash
// } from '@holochain/client';

// import { EntryRecord } from '@holochain-open-dev/utils';
// import { UnsubscribeFunction } from 'emittery';
import type {
	ActionHash,
	AgentPubKey,
	AppAgentCallZomeRequest,
	AppAgentClient,
	HoloHashed
} from '@holochain/client';

type KeyAnchor = { bytes: Uint8Array };
export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
	return { bytes: pubkey.slice(3, 35) };
}

type Authorization = [number, Buffer]; // u8 index, 64 byte signature

type KeyGeneration = {
	new_key: AgentPubKey;
	new_key_signing_of_author: Buffer; // 64 bytes
};

type KeyRevocation = {
	prior_key_registration: ActionHash;
	revocation_authorization: Authorization[];
};

type KeyRegistration =
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

	//   on(
	//     eventName: 'signal',
	//     listener: (eventData: HeardSignal) => void | Promise<void>
	//   ): UnsubscribeFunction {
	//     return this.client.on(eventName, async signal => {
	//       if (
	//         (await isSignalFromCellWithRole(this.client, this.roleName, signal)) &&
	//         this.zomeName === signal.zome_name
	//       ) {
	//         listener(signal.payload as HeardSignal);
	//       }
	//     });
	//   }

	async key_state(agentKey: AgentPubKey): Promise<KeyState> {
		return this.callZome('key_state', [agentKey, Date.now()]);
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

	async query_keyset_keys(ksr: ActionHash): Promise<KeyAnchor[]> {
		return this.callZome('query_keyset_keys', ksr);
	}
	
	async invite_agent(agentKey: AgentPubKey): Promise<DeviceInviteAcceptance> {
		return this.callZome('invite_agent', agentKey);
	}
	async accept_invitation(dia: DeviceInviteAcceptance): Promise<ActionHash> {
		return this.callZome('accept_invite', dia);
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
