import { DeepkeyClient } from '$lib/deepkey-client';
import {
	authorizeClient,
	locallyAuthorizeSigningCredentials,
	setupHolochain
} from '$lib/holochain-client';
import {
	AppAgentWebsocket,
	CellType,
	type CellId,
	type SigningCredentials
} from '@holochain/client';
import { derived, readable, writable, type Readable, get } from 'svelte/store';

const HOLOCHAIN_APP_ID = 'deepkey';
// const HOLOCHAIN_URL = new URL(`ws://localhost:${import.meta.env.VITE_HC_PORT}`);

export type SelfAuthorizedHolochainWebsocket = AppAgentWebsocket & {
	cellId: CellId;
	creds: SigningCredentials | undefined;
};

function createHolochainClientStore() {
	const initialValue = undefined;
	const { subscribe, set } = writable<SelfAuthorizedHolochainWebsocket>(initialValue);
	let initComplete = false;

	return {
		subscribe,
		async init() {
			const client = await setupHolochain();
			// const client = await AppAgentWebsocket.connect(HOLOCHAIN_URL, HOLOCHAIN_APP_ID, 60000);

			const appInfo = await client.appInfo();
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore
			const { cell_id: cellId } = appInfo.cell_info[HOLOCHAIN_APP_ID][0][CellType.Provisioned];
			const creds = await authorizeClient(cellId);

			// Just store these right on the client.
			// If we were communicating with multiple Zomes, we'd need this to be indexed by the Zome.
			// e.g. client.cell['deepkey']; client.creds['deepkey']
			// But we're just doing this for deepkey right now.
			(client as SelfAuthorizedHolochainWebsocket).cellId = cellId;
			(client as SelfAuthorizedHolochainWebsocket).creds = creds;

			set(client as SelfAuthorizedHolochainWebsocket);
			initComplete = true;
			return client as SelfAuthorizedHolochainWebsocket;
		},
		get initComplete() {
			return initComplete;
		}
	};
}

export const holochain = createHolochainClientStore();

// export const appInfo = derived(
// 	holochain,
// 	($holochain, set) => {
// 		$holochain.appInfo().then((value) => {
// 			set(value);
// 		});
// 	}
// );

// export const deepkeyAgentPubkey = derived(appInfo, ($appInfo) => $appInfo.agent_pub_key);
