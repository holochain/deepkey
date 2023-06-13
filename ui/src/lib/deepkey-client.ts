// import type {
// 	ActionHash,
// 	AgentPubKey,
// 	AppAgentCallZomeRequest,
// 	AppAgentClient,
// 	EntryHash,
// 	HoloHash
// } from '@holochain/client';

// import { EntryRecord } from '@holochain-open-dev/utils';
// // import { UnsubscribeFunction } from 'emittery';
// import { AgentPubKey } from '@holochain/client';

// type KeyAnchor = UIntArray[32];
// export function getKeyAnchor(pubkey: AgentPubKey): KeyAnchor {
// 	return pubkey.slice(0, 32);
// }

// export class DeepkeyClient {
// 	constructor(
// 		public client: AppAgentClient,
// 		public roleName: string,
// 		public zomeName = 'deepkey'
// 	) {}

// 	//   on(
// 	//     eventName: 'signal',
// 	//     listener: (eventData: HeardSignal) => void | Promise<void>
// 	//   ): UnsubscribeFunction {
// 	//     return this.client.on(eventName, async signal => {
// 	//       if (
// 	//         (await isSignalFromCellWithRole(this.client, this.roleName, signal)) &&
// 	//         this.zomeName === signal.zome_name
// 	//       ) {
// 	//         listener(signal.payload as HeardSignal);
// 	//       }
// 	//     });
// 	//   }

// 	async key_state(keyAnchor: KeyAnchor): Promise<KeyState> {
// 		return this.callZome('key_state', keyAnchor);
// 	}

// 	// eslint-disable-next-line @typescript-eslint/no-explicit-any
// 	callZome(fn_name: string, payload: any) {
// 		const req: AppAgentCallZomeRequest = {
// 			role_name: this.roleName,
// 			zome_name: this.zomeName,
// 			fn_name,
// 			payload
// 		};
// 		return this.client.callZome(req, 30000);
// 	}
// }

export default { DeepkeyClient: {} };
