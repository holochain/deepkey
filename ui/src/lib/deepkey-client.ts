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
import { type AgentPubKey } from '@holochain/client';

// type KeyAnchor = UInt8Array[32];
// export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
// 	return pubkey.slice(4, 36);
// }

export class DeepkeyClient {
	constructor(
		public client: AppAgentClient,
		// public roleName: string,
		public cellId,
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

	// eslint-disable-next-line @typescript-eslint/no-explicit-any
	callZome(fn_name: string, payload: any) {
		const req: AppAgentCallZomeRequest = {
			// role_name: this.roleName,
			cell_id: this.cellId,
			zome_name: this.zomeName,
			fn_name,
			payload
		};
		return this.client.callZome(req, 30000);
	}

}

// export const DeepkeyClient = {};
